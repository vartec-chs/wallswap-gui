import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import {
	Carousel,
	CarouselContent,
	CarouselItem,
	CarouselNext,
	CarouselPrevious,
} from '@/components/ui/carousel'
import { ScrollArea, ScrollBar } from '@/components/ui/scroll-area'
import { HistoryIcon, Loader } from 'lucide-react'
import { type FC } from 'react'

export const works = [
	{
		artist: 'Ornella Binni',
		art: 'https://images.unsplash.com/photo-1465869185982-5a1a7522cbcb?auto=format&fit=crop&w=300&q=80',
	},
	{
		artist: 'Tom Byrom',
		art: 'https://images.unsplash.com/photo-1548516173-3cabfa4607e9?auto=format&fit=crop&w=300&q=80',
	},
	{
		artist: 'Vladimir Malyavko',
		art: 'https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80',
	},
	{
		artist: 'Vladimir Malyavko',
		art: 'https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80',
	},
	{
		artist: 'Vladimir Malyavko',
		art: 'https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80',
	},
	{
		artist: 'Vladimir Malyavko',
		art: 'https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80',
	},
]

export const PicturesSection: FC = () => {
	return (
		<div className='flex flex-col w-full h-fit  items-start justify-between gap-4 border p-2 rounded-lg shadow-lg bg-white dark:bg-neutral-900'>
			<div className='flex w-full items-center justify-between gap-1'>
				<h1 className='text-lg font-bold'>Область изображений</h1>
				<Button variant='ghost' size='icon' className='w-8 h-8 p-0'>
					<HistoryIcon />
				</Button>
			</div>
			<div className='w-full'>
				{/* <Loader className='w-4 h-4 animate-spin text-gray-500' /> */}
				<ScrollArea className='w-full h-full '>
					<div className='flex w-max space-x-4 '>
						{works.map((artwork) => (
							<figure key={artwork.artist} className='shrink-0'>
								<div className='overflow-hidden rounded-md'>
									<img
										src={artwork.art}
										alt={`Photo by ${artwork.artist}`}
										className='aspect-[3/4] h-[160px] w-[170px] object-cover'
										width={170}
										height={160}
									/>
								</div>
								<figcaption className='text-muted-foreground pt-2 text-xs w-[160px] truncate'>
									Photo by <span className='text-foreground font-semibold'>{artwork.artist}</span>
								</figcaption>
							</figure>
						))}
					</div>
					<ScrollBar orientation='horizontal' />
				</ScrollArea>
			</div>
			{/* Здесь можно добавить галерею изображений или другие элементы */}
		</div>
	)
}
