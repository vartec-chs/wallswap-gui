import { CollectWallpaperAssemblyModal } from './CollectWallpaperAssemblyModal'
import { useTest } from './hooks/useTest'
import { RandomWallpaperModal } from './RandomWallpaperModal'
import { Button } from '@/components/ui/button'
import { type FC } from 'react'

export const ActionSection: FC = () => {
	// Здесь можно добавить логику для обработки действий, если необходимо

	// const { answer, loading, error, refetch } = useTest()

	// console.log('testData', answer?.code)

	// if (loading) {
	// 	return <div>Загрузка...</div>
	// }

	// if (error) {
	// 	return <div>{error.message}</div>
	// }

	return (
		<div className='flex flex-col w-full  items-start justify-between gap-4 border p-2 rounded-lg shadow-lg bg-white dark:bg-neutral-900'>
			<div className='flex w-full items-center justify-between gap-1'>
				<RandomWallpaperModal />
				<CollectWallpaperAssemblyModal />
				{/* <Button variant='outline' onClick={refetch}>
					Действие 3
				</Button> */}
			</div>
			{/* {testData?.code} */}
			{/* Здесь можно добавить кнопки или другие элементы управления */}
		</div>
	)
}
