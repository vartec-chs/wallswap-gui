import { Button } from '@/components/ui/button'
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@radix-ui/react-label'
import { Settings } from 'lucide-react'
import { type FC, useState } from 'react'

export type SettingsSectionProps = {
	isOpen?: boolean
}

export const SettingsSection: FC<SettingsSectionProps> = ({ isOpen = false }) => {
	const [open, setOpen] = useState(isOpen)

	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault()
		// Handle form submission logic here
		console.log('Form submitted')
	}
	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<form onSubmit={handleSubmit}>
				<DialogTrigger asChild>
					<Button size='icon' variant='ghost' className='w-8 h-8 p-0'>
						<Settings
							className={`transition-transform duration-200 ${open ? 'rotate-90' : 'rotate-0'}`}
						/>
					</Button>
				</DialogTrigger>
				<DialogContent className='sm:max-w-[425px]'>
					<DialogHeader>
						<DialogTitle>Edit profile</DialogTitle>
						<DialogDescription>
							Make changes to your profile here. Click save when you&apos;re done.
						</DialogDescription>
					</DialogHeader>
					<div className='grid gap-4'>
						<div className='grid gap-3'>
							<Label htmlFor='name-1'>Name</Label>
							<Input id='name-1' name='name' defaultValue='Pedro Duarte' />
						</div>
						<div className='grid gap-3'>
							<Label htmlFor='username-1'>Username</Label>
							<Input id='username-1' name='username' defaultValue='@peduarte' />
						</div>
					</div>
					<DialogFooter>
						<DialogClose asChild>
							<Button variant='outline'>Cancel</Button>
						</DialogClose>
						<Button type='submit'>Save changes</Button>
					</DialogFooter>
				</DialogContent>
			</form>
		</Dialog>
	)
}
