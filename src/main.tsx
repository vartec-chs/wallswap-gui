import { App } from './App'
import { ThemeProvider } from '@/components/providers/theme-provider'
import { Toaster } from '@/components/ui/sonner'
import '@/index.css'
import ReactDOM from 'react-dom/client'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<ThemeProvider defaultTheme='system' storageKey='wallswap-theme'>
		<App />
		<Toaster />
	</ThemeProvider>,
)
