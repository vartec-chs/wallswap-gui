import { testService } from '@/api/services/test.service'
import { useApiCall } from '@/hooks/useApiCall'
import { useEffect } from 'react'

export function useTest() {
	const { answer, loading, error, execute } = useApiCall<string>()

	useEffect(() => {
		execute(() => testService.getTestData())
	}, [execute])

	const refetch = () => {
		execute(() => testService.getTestData())
	}

	return { answer, loading, error, refetch }
}
