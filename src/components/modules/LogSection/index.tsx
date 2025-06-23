import { ScrollArea } from '@/components/ui/scroll-area'
import { Loader } from 'lucide-react'
import { type FC } from 'react'

const logs = [
	'Log entry 1: Application started successfully.',
	'Log entry 2: User logged in.',
	'Log entry 3: Data fetched from API.',
	'Log entry 4: User updated profile information.',
	'Log entry 5: Error occurred while processing request.',
	'Log entry 6: User logged out.',
	'Log entry 7: Application closed.',
]

export const LogSection: FC = () => {
	return (
		<div className='flex flex-col  w-[50%] flex-1/2 h-full items-start justify-between gap-1 border p-2 rounded-lg shadow-lg bg-white dark:bg-neutral-900'>
			<div className='flex w-full items-center justify-between gap-1'>
				<h1 className='text-lg font-bold'>Логи</h1>
				{/* <Loader className='w-4 h-4 animate-spin text-gray-500' /> */}
			</div>
			<ScrollArea className='w-full h-full'>
				<div className='flex flex-col w-full h-full items-start justify-between gap-1'>
					{logs.map((log, index) => (
						<p key={index} className='text-sm text-gray-700 dark:text-gray-300'>
							{log}
						</p>
					))}
				</div>
			</ScrollArea>
			{/* <div className='flex flex-row w-full items-start justify-between gap-1'>
				<p className='text-sm text-gray-500'></p>
			</div> */}
		</div>
	)
}
