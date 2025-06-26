import { invoke } from '@tauri-apps/api/core'

export type ApiResult<T> =
	| { type: 'success'; answer: ApiSuccess<T> }
	| { type: 'error'; answer: ApiError }

// "category": "general",
//           "error_type": "other",
//           "message": "Test error",
//           "retryable": false

export type ErrorDetails = {
	category: string
	error_type: string
	retryable?: boolean
	[key: string]: any
}

export interface ApiError {
	code: ErrorCode
	message: string
	details?: ErrorDetails
}

export interface ApiSuccess<T> {
	code: SuccessCode
	data: T
}

export enum SuccessCode {
	OPERATION_SUCCESSFUL = 'OPERATION_SUCCESSFUL',
	DATA_RETRIEVED = 'DATA_RETRIEVED',
	USER_CREATED = 'USER_CREATED',
	USER_UPDATED = 'USER_UPDATED',
	USER_DELETED = 'USER_DELETED',
	FILE_UPLOADED = 'FILE_UPLOADED',
}

export enum ErrorCode {
	// Общие ошибки
	INTERNAL_ERROR = 'INTERNAL_ERROR',
	INVALID_INPUT = 'INVALID_INPUT',
	NOT_FOUND = 'NOT_FOUND',
	UNAUTHORIZED = 'UNAUTHORIZED',
	FORBIDDEN = 'FORBIDDEN',

	// Пользовательские ошибки
	USER_NOT_FOUND = 'USER_NOT_FOUND',
	USER_ALREADY_EXISTS = 'USER_ALREADY_EXISTS',
	INVALID_CREDENTIALS = 'INVALID_CREDENTIALS',

	// Файловые ошибки
	FILE_NOT_FOUND = 'FILE_NOT_FOUND',
	FILE_ACCESS_DENIED = 'FILE_ACCESS_DENIED',
	FILE_CORRUPTED = 'FILE_CORRUPTED',

	// Сетевые ошибки
	NETWORK_ERROR = 'NETWORK_ERROR',
	TIMEOUT = 'TIMEOUT',

	// Бизнес-логика
	VALIDATION_FAILED = 'VALIDATION_FAILED',
	OPERATION_FAILED = 'OPERATION_FAILED',
}

// utils/result.ts
export class Result<T> {
	private constructor(
		private readonly isSuccess: boolean,
		private readonly value?: ApiSuccess<T>,
		private readonly error?: ApiError,
	) {}

	static success<T>(data: ApiSuccess<T>): Result<T> {
		return new Result<T>(true, data)
	}

	static error<T>(error: ApiError): Result<T> {
		return new Result<T>(false, undefined, error)
	}

	static fromApiResult<T>(apiResult: ApiResult<T>): Result<T> {
		if (apiResult.type === 'success') {
			return Result.success(apiResult.answer)
		} else {
			console.error('API Error:', apiResult.answer)
			return Result.error(apiResult.answer)
		}
	}

	isOk(): boolean {
		return this.isSuccess
	}

	isErr(): boolean {
		return !this.isSuccess
	}

	unwrap(): ApiSuccess<T> {
		if (!this.isSuccess) {
			throw new Error(`Called unwrap on error: ${this.error?.message}`)
		}
		return this.value!
	}

	unwrapOr(defaultValue: ApiSuccess<T>): ApiSuccess<T> {
		return this.isSuccess ? this.value! : defaultValue
	}

	unwrapOrElse(fn: (error: ApiError) => ApiSuccess<T>): ApiSuccess<T> {
		return this.isSuccess ? this.value! : fn(this.error!)
	}

	map<U>(fn: (value: ApiSuccess<T>) => ApiSuccess<U>): Result<U> {
		if (this.isSuccess) {
			return Result.success(fn(this.value!))
		} else {
			return Result.error(this.error!)
		}
	}

	mapErr(fn: (error: ApiError) => ApiError): Result<T> {
		if (this.isSuccess) {
			return new Result<T>(true, this.value!)
		} else {
			return Result.error(fn(this.error!))
		}
	}

	andThen<U>(fn: (value: ApiSuccess<T>) => Result<U>): Result<U> {
		if (this.isSuccess) {
			return fn(this.value!)
		} else {
			return Result.error(this.error!)
		}
	}

	match<U>(onSuccess: (value: ApiSuccess<T>) => U, onError: (error: ApiError) => U): U {
		if (this.isSuccess) {
			return onSuccess(this.value!)
		} else {
			return onError(this.error!)
		}
	}

	getError(): ApiError | undefined {
		return this.error
	}
}

export class TauriAPI {
	static async execCommand<T>(command: string, args?: Record<string, any>): Promise<Result<T>> {
		try {
			const apiResult = await invoke<ApiResult<T>>(command, args)
			console.log('API Result:', apiResult)
			return Result.fromApiResult(apiResult)
		} catch (error) {
			// Обработка ошибок от Tauri
			console.error('Error invoking Tauri command:', error)
			return Result.error({
				code: 'INTERNAL_ERROR' as any,
				message: error instanceof Error ? error.message : 'Unknown error',
			})
		}
	}
}
