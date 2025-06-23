import { useTheme } from '../providers/theme-provider'
import { Button } from '@/components/ui/button'
import { MoonIcon, SunIcon } from 'lucide-react'

export function ToggleThemeButton() {
	const { theme, setTheme } = useTheme()

	return (
		<Button
			variant='ghost'
			size='icon'
			onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
			className='w-8 h-8 p-0'
		>
			{theme === 'dark' ? <SunIcon className='w-4 h-4' /> : <MoonIcon className='w-4 h-4' />}
		</Button>
	)
}
