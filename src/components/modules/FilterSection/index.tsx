import { SettingsSection } from '../SettingsSection'
import { Filters } from './filters'
import { ToggleThemeButton } from '@/components/shared/toggle-theme'
import { type FC } from 'react'

export const FilterSection: FC = () => {
	return (
		<div className='flex flex-col w-full h-full flex-1/2 items-start justify-between gap-4 border p-2 rounded-lg shadow-lg bg-white dark:bg-neutral-900'>
			<div className='flex w-full items-center justify-between gap-1'>
				<h1 className='text-lg font-bold'>Фильтры</h1>
				<div className='flex items-center gap-2'>
					<ToggleThemeButton /> <SettingsSection />
				</div>
			</div>

			<Filters />
		</div>
	)
}
