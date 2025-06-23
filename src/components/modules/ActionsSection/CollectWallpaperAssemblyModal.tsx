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
import { Label } from '@/components/ui/label'
import { NumberInput } from '@/components/ui/number-input'
import { type FC, useState } from 'react'

export type CollectWallpaperAssemblyModalProps = {
	isOpen?: boolean
	onClose?: () => void
	onSubmit?: (assemblyName: string) => void
}

export const CollectWallpaperAssemblyModal: FC<CollectWallpaperAssemblyModalProps> = ({
	isOpen,
	onClose,
	onSubmit,
}) => {
	const [open, setOpen] = useState(isOpen ?? false)
	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<form
				onSubmit={(event) => {
					event.preventDefault()
					if (onSubmit) {
						const formData = new FormData(event.currentTarget)
						const assemblyName = formData.get('assemblyName') as string
						onSubmit(assemblyName)
					}
					setOpen?.(false)
				}}
			>
				<DialogTrigger asChild>
					<Button variant='outline'>Собрать сборку обоев</Button>
				</DialogTrigger>
				<DialogContent className='sm:max-w-[425px]'>
					<DialogHeader>
						<DialogTitle>Собрать сборку обоев</DialogTitle>
					</DialogHeader>
					<div className='grid gap-4'>
						<div className='grid gap-3'>
							<Label htmlFor='count'>Кол-во обоев</Label>
							<NumberInput id='count' name='count' defaultValue={1} />
						</div>
						{/* <div className='grid gap-3'>
							<Label htmlFor='username-1'>Username</Label>
							<Input id='username-1' name='username' defaultValue='@peduarte' />
						</div> */}
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
