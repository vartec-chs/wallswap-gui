import { Label } from '@/components/ui/label'
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'
import { type FC } from 'react'

const screenResolutions = [
	{ label: '1920x1080', value: '1920x1080' },
	{ label: '2560x1440', value: '2560x1440' },
	{ label: '3840x2160', value: '3840x2160' },
	{ label: '1280x720', value: '1280x720' },
	{ label: '1024x768', value: '1024x768' },
	{ label: '800x600', value: '800x600' },
	{ label: '640x480', value: '640x480' },
	{ label: '320x240', value: '320x240' },
	{ label: '160x120', value: '160x120' },
	{ label: 'Custom', value: 'custom' },
]

const categories = [
	{ label: 'Nature', value: 'nature' },
	{ label: 'Technology', value: 'technology' },
	{ label: 'People', value: 'people' },
	{ label: 'Architecture', value: 'architecture' },
	{ label: 'Food', value: 'food' },
	{ label: 'Travel', value: 'travel' },
	{ label: 'Animals', value: 'animals' },
	{ label: 'Sports', value: 'sports' },
	{ label: 'Fashion', value: 'fashion' },
	{ label: 'Art', value: 'art' },
]

const wallpaperSuppliers = [
	{
		name: 'WallpapersHub',
		url: 'https://wallpapershub.app',
		description: 'A collection of high-quality wallpapers for your desktop and mobile devices.',
		license: 'CC0 1.0 Universal (CC0 1.0) Public Domain Dedication',
	},
	{
		name: 'Unsplash',
		url: 'https://unsplash.com',
		description: 'A platform for free high-resolution photos and wallpapers.',
		license: 'Unsplash License',
	},
]

export const Filters: FC = () => {
	return (
		<div className='flex flex-col w-full not-visited:items-start justify-between gap-1'>
			<div className='flex items-center w-full justify-between gap-2'>
				<Switch id='screen-resolution' className='mr-2' />
				<div className='flex-1 gap-1 flex flex-col'>
					<Label htmlFor='screen-resolution' className='text-sm'>
						Выберите разрешение экрана
					</Label>
					<Select defaultValue='1920x1080'>
						<SelectTrigger className='w-full'>
							<SelectValue placeholder='Выберите разрешение' />
						</SelectTrigger>
						<SelectContent id='screen-resolution'>
							<SelectGroup>
								<SelectLabel>Разрешения экрана</SelectLabel>
								{screenResolutions.map((resolution) => (
									<SelectItem key={resolution.value} value={resolution.value}>
										{resolution.label}
									</SelectItem>
								))}
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>
			</div>
			<Label htmlFor='category' className='text-sm'>
				Выберите категорию
			</Label>
			<Select>
				<SelectTrigger className='w-full'>
					<SelectValue placeholder='Выберите категорию' />
				</SelectTrigger>
				<SelectContent id='category'>
					<SelectGroup>
						<SelectLabel>Categories</SelectLabel>
						{categories.map((category) => (
							<SelectItem key={category.value} value={category.value}>
								{category.label}
							</SelectItem>
						))}
					</SelectGroup>
				</SelectContent>
			</Select>

			<Label htmlFor='wallpaper-supplier' className='text-sm'>
				Выберите поставщика обоев
			</Label>
			<Select>
				<SelectTrigger
					style={{
						textOverflow: 'ellipsis',
						overflow: 'hidden',
						whiteSpace: 'nowrap',
						maxWidth: '400px',
					}}
					className='w-full'
				>
					<SelectValue placeholder='Выберите поставщика обоев' />
				</SelectTrigger>
				<SelectContent id='wallpaper-supplier'>
					<SelectGroup>
						<SelectLabel>Поставщики обоев</SelectLabel>
						{wallpaperSuppliers.map((supplier) => (
							<SelectItem key={supplier.name} value={supplier.name}>
								{supplier.name} - {supplier.description}
							</SelectItem>
						))}
					</SelectGroup>
				</SelectContent>
			</Select>
		</div>
	)
}
