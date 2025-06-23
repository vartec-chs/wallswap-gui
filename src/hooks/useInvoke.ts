import { invoke } from '@tauri-apps/api/core'
import { useCallback, useEffect, useRef, useState } from 'react'

type ApiResult<T = unknown> = {
	data: T
	message: string
	statusCode: number
	isSuccess: boolean
}

type InvokeArgs = {
	[key: string]: any
}

type InvokeError<T = unknown> = ApiResult<T>

export type BaseInvokeOptions<Return> = {
	runOnMount?: boolean
	onSuccess?: (data: ApiResult<Return>) => void
	onError?: (error: InvokeError<Return>) => void
	onFinished?: () => void
}

interface UseInvokeOptions<Return, Params extends InvokeArgs = {}>
	extends BaseInvokeOptions<Return> {
	command: string
	invokeArgs?: Params
	enabled?: boolean
	timeout?: number
}

interface UseInvokeReturn<Return, Params extends InvokeArgs = {}> {
	data: ApiResult<Return> | undefined
	isLoading: boolean
	error: InvokeError<Return> | undefined
	execute: (invokeArgs?: Params) => Promise<ApiResult<Return>>
	cancel: () => void
}

// Дополнительные типы для работы с новой системой ошибок

interface ErrorDetails {
	category: string
	error_type: string
	retryable?: boolean
	[key: string]: any
}

interface ApiError {
	code: string
	message: string
	details?: any
}

interface ExtendedApiError extends ApiError {
	details: ErrorDetails | null
}

interface StructuredError {
	message: string
	statusCode: number
}

interface ExtendedInvokeError<T> extends InvokeError<T> {
	details?: ErrorDetails
}

interface ExtendedUseInvokeReturn<Return, Params extends InvokeArgs = {}>
	extends UseInvokeReturn<Return, Params> {
	retryCount: number
}

// Улучшенная функция обработки ошибок
export const enhancedErrorHandler = <Return = never>(error: unknown): InvokeError<Return> => {
	// Null и undefined
	if (error == null) {
		return {
			message: 'Null or undefined error',
			statusCode: 500,
			isSuccess: false,
			data: undefined as never,
		}
	}

	// Строковые ошибки
	if (typeof error === 'string') {
		return {
			message: error,
			statusCode: 500,
			isSuccess: false,
			data: undefined as never,
		}
	}

	// Error объекты
	if (error instanceof Error) {
		return {
			message: error.message,
			statusCode: error.name === 'TimeoutError' ? 408 : 500,
			isSuccess: false,
			data: undefined as never,
		}
	}

	// Структурированные ошибки от API
	if (
		typeof error === 'object' &&
		error !== null &&
		'message' in error &&
		'statusCode' in error &&
		typeof (error as StructuredError).message === 'string' &&
		typeof (error as StructuredError).statusCode === 'number'
	) {
		return error as ApiResult<Return>
	}

	// Попытка извлечь сообщение из объекта
	let message = 'Unknown error occurred'
	if (typeof error === 'object' && error !== null) {
		if ('message' in error && typeof (error as any).message === 'string') {
			message = (error as any).message
		} else if ('toString' in error) {
			message = (error as any).toString()
		}
	}

	return {
		message,
		statusCode: 500,
		isSuccess: false,
		data: undefined as never,
	}
}

// Хелперы для работы с категориями ошибок
export const isNetworkError = (error: ExtendedApiError): boolean => {
	return error.details?.category === 'network'
}

export const isFileSystemError = (error: ExtendedApiError): boolean => {
	return error.details?.category === 'file_system'
}

export const isRetryableError = (error: ExtendedApiError): boolean => {
	return error.details?.retryable === true
}

// Хук для автоматического retry
export const useInvokeWithRetry = <Return, Params extends InvokeArgs = {}>({
	command,
	invokeArgs: defaultInvokeArgs,
	runOnMount = false,
	onSuccess,
	onError,
	onFinished,
	enabled = true,
	timeout,
	maxRetries = 3,
	retryDelay = 1000,
}: UseInvokeOptions<Return, Params> & {
	maxRetries?: number
	retryDelay?: number
}): UseInvokeReturn<Return, Params> => {
	const [retryCount, setRetryCount] = useState(0)

	const executeWithRetry = useCallback(
		async (invokeArgs?: Params, currentRetry = 0): Promise<ApiResult<Return>> => {
			try {
				setIsLoading(true)
				setError(undefined)
				const finalArgs = invokeArgs ?? defaultInvokeArgs ?? {}

				const result = await invoke<ApiResult<Return>>(command, finalArgs)

				setData(result)
				setRetryCount(0)
				onSuccess?.(result)
				return result
			} catch (err) {
				const error = enhancedErrorHandler<Return>(err)

				// Проверяем, можно ли повторить
				if (currentRetry < maxRetries && (error as any).details?.retryable) {
					console.log(`Retry attempt ${currentRetry + 1} for command: ${command}`)
					setRetryCount(currentRetry + 1)

					// Экспоненциальная задержка
					await new Promise((resolve) =>
						setTimeout(resolve, retryDelay * Math.pow(2, currentRetry)),
					)

					return executeWithRetry(invokeArgs, currentRetry + 1)
				}

				setError(error)
				onError?.(error)
				throw error
			} finally {
				setIsLoading(false)
				onFinished?.()
			}
		},
		[command, defaultInvokeArgs, onSuccess, onError, onFinished, maxRetries, retryDelay],
	)

	const [data, setData] = useState<ApiResult<Return> | undefined>(undefined)
	const [isLoading, setIsLoading] = useState(false)
	const [error, setError] = useState<InvokeError<Return> | undefined>(undefined)
	const abortControllerRef = useRef<AbortController | null>(null)

	const execute = useCallback(
		async (invokeArgs?: Params): Promise<ApiResult<Return>> => {
			setIsLoading(true)
			setError(undefined)
			const finalArgs = invokeArgs ?? defaultInvokeArgs ?? {}
			abortControllerRef.current = new AbortController()

			try {
				const result = await Promise.race([
					invoke<ApiResult<Return>>(command, finalArgs),
					...(timeout
						? [
								new Promise<never>((_, reject) =>
									setTimeout(() => reject(new Error('Timeout')), timeout),
								),
							]
						: []),
				])
				setData(result)
				onSuccess?.(result)
				return result
			} catch (err) {
				const error = errorHandlerType<Return>(err)
				setError(error)
				onError?.(error)
				throw error
			} finally {
				setIsLoading(false)
				onFinished?.()
				abortControllerRef.current = null
			}
		},
		[command, defaultInvokeArgs, onSuccess, onError, onFinished, timeout],
	)

	const cancel = useCallback(() => {
		if (abortControllerRef.current) {
			abortControllerRef.current.abort()
		}
	}, [])

	useEffect(() => {
		if (runOnMount && enabled) {
			execute().catch((err) => {
				console.error('Self-calling invoke failed:', err)
			})
		}
		return () => {
			cancel()
		}
	}, [runOnMount, enabled, execute, cancel])

	return {
		data,
		isLoading,
		error,
		execute: executeWithRetry,
		cancel,
		retryCount, // Добавляем счетчик повторов
	} as ExtendedUseInvokeReturn<Return, Params>
}

export const errorHandlerType = <Return = never>(error: unknown): InvokeError<Return> => {
	if (typeof error === 'string') {
		return {
			message: error,
			statusCode: 500,
			isSuccess: false,
			data: undefined as never,
		}
	}
	if (error instanceof Error) {
		return {
			message: error.message,
			statusCode: 500,
			isSuccess: false,
			data: undefined as never,
		}
	}
	if (typeof error === 'object' && error !== null && 'message' in error && 'statusCode' in error) {
		return error as ApiResult<Return>
	}
	return {
		message: 'Unknown error occurred',
		statusCode: 500,
		isSuccess: false,
		data: undefined as never,
	}
}
