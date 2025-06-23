import { Button } from '@/components/ui/button'
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '@/components/ui/dialog'
import { type FC, useState } from 'react'

export type RandomWallpaperModalProps = {
	isOpen?: boolean
	onClose?: () => void
	onSubmit?: (assemblyName: string) => void
}

export const RandomWallpaperModal: FC<RandomWallpaperModalProps> = ({
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
					<Button variant='default'>Случайные обои</Button>
				</DialogTrigger>
				<DialogContent className='sm:max-w-[425px]'>
					<DialogHeader>
						<DialogTitle>Собрать сборку обоев</DialogTitle>
					</DialogHeader>

					<DialogFooter>
						<DialogClose asChild>
							<Button variant='outline'>Отмена</Button>
						</DialogClose>
						<Button type='submit'>Установить</Button>
					</DialogFooter>
				</DialogContent>
			</form>
		</Dialog>
	)
}
