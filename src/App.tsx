import { ActionSection } from './components/modules/ActionsSection'
import { FilterSection } from './components/modules/FilterSection'
import { LogSection } from './components/modules/LogSection'
import { PicturesSection } from './components/modules/PicturesSection'
import { ToggleThemeButton } from './components/shared/toggle-theme'

export const App = () => {
	return (
		<div className='flex p-1 gap-1 flex-col h-screen w-screen items-start justify-between '>
			<div className='flex w-full flex-col  items-center justify-between gap-1 '>
				<div className='flex w-full items-center justify-between gap-1 '>
					<FilterSection />
					<LogSection />
				</div>
				<ActionSection />
			</div>
			<PicturesSection />
		</div>
	)
}
