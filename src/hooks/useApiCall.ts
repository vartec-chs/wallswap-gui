import { ApiError, ApiSuccess, Result } from '@/utils/api-result'
import { useCallback, useState } from 'react'

interface ApiCallState<T> {
	answer: ApiSuccess<T> | null
	loading: boolean
	error: ApiError | null
}

export function useApiCall<T>() {
	const [state, setState] = useState<ApiCallState<T>>({
		answer: null,
		loading: false,
		error: null,
	})

	const execute = useCallback(async (apiCall: () => Promise<Result<T>>) => {
		setState((prev) => ({ ...prev, loading: true, error: null }))

		try {
			const result = await apiCall()

			console.log(result)

			result.match(
				(answer) => setState({ answer, loading: false, error: null }),
				(error) => setState({ answer: null, loading: false, error }),
			)

			return result
		} catch (error) {
			const apiError: ApiError = {
				code: 'INTERNAL_ERROR' as any,
				message: error instanceof Error ? error.message : 'Unknown error',
			}
			setState({ answer: null, loading: false, error: apiError })
			return Result.error<T>(apiError)
		}
	}, [])

	const reset = useCallback(() => {
		setState({ answer: null, loading: false, error: null })
	}, [])

	return { ...state, execute, reset }
}
