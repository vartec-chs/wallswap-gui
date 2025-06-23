import { Result, TauriAPI } from '@/utils/api-result'

class TestService {
	async getTestData(): Promise<Result<string>> {
		return TauriAPI.execCommand<string>('test_command')
	}
}

export const testService = new TestService()
