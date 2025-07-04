import { invoke } from '@tauri-apps/api/core'
import { error } from 'console'

export type AppResult<T> = { Ok: AppSuccess<T> } | { Err: AppError }

export type AppSuccess<T> = {
	code?: string
	message?: string
	data?: T
	timestamp?: string // ISO 8601 format
	execution_time_ms?: number
	metadata?: Record<string, any>
}

export type ErrorDetails = {
	category: string
	error_type: string
	severity: string
	retryable?: boolean
	message: string
	full_message?: string
	timestamp?: string // ISO 8601 format
	trace_id?: string
	nested_details?: Record<string, any>
}

export type AppError = ErrorDetails

// utils/result.ts
export class Result<T> {
	private constructor(
		private readonly isSuccess: boolean,
		private readonly value?: AppSuccess<T>,
		private readonly error?: AppError,
	) {}

	static success<T>(data: AppSuccess<T>): Result<T> {
		return new Result<T>(true, data)
	}

	static error<T>(error: AppError): Result<T> {
		return new Result<T>(false, undefined, error)
	}

	static fromAppResult<T>(appResult: AppResult<T>): Result<T> {
		if ('Ok' in appResult) {
			return Result.success(appResult.Ok)
		} else {
			console.error('App Error:', appResult.Err)
			return Result.error(appResult.Err)
		}
	}

	isOk(): boolean {
		return this.isSuccess
	}

	isErr(): boolean {
		return !this.isSuccess
	}

	unwrap(): AppSuccess<T> {
		if (!this.isSuccess) {
			throw new Error(`Called unwrap on error: ${this.error?.message}`)
		}
		return this.value!
	}

	unwrapOr(defaultValue: AppSuccess<T>): AppSuccess<T> {
		return this.isSuccess ? this.value! : defaultValue
	}

	unwrapOrElse(fn: (error: AppError) => AppSuccess<T>): AppSuccess<T> {
		return this.isSuccess ? this.value! : fn(this.error!)
	}

	map<U>(fn: (value: AppSuccess<T>) => AppSuccess<U>): Result<U> {
		if (this.isSuccess) {
			return Result.success(fn(this.value!))
		} else {
			return Result.error(this.error!)
		}
	}

	mapErr(fn: (error: AppError) => AppError): Result<T> {
		if (this.isSuccess) {
			return new Result<T>(true, this.value!)
		} else {
			return Result.error(fn(this.error!))
		}
	}

	andThen<U>(fn: (value: AppSuccess<T>) => Result<U>): Result<U> {
		if (this.isSuccess) {
			return fn(this.value!)
		} else {
			return Result.error(this.error!)
		}
	}

	match<U>(onSuccess: (value: AppSuccess<T>) => U, onError: (error: AppError) => U): U {
		if (this.isSuccess) {
			return onSuccess(this.value!)
		} else {
			return onError(this.error!)
		}
	}

	getError(): AppError | undefined {
		return this.error
	}
}

export class TauriAPI {
	static async execCommand<T>(command: string, args?: Record<string, any>): Promise<Result<T>> {
		try {
			const appResult = await invoke<AppResult<T>>(command, args)
			console.log('App Result:', appResult)
			return Result.fromAppResult(appResult)
		} catch (error: any) {
			return Result.error(error as AppError)
		}
	}
}
