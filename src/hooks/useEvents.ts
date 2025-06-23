import { Channel } from '@tauri-apps/api/core'
import { emit, emitTo, type Event, listen, once } from '@tauri-apps/api/event'
import { useCallback, useEffect, useState } from 'react'

// 1. Добавляем зависимости в useEffect для корректной работы
export const useListen = <Return>(
	eventName: string,
	callback: (event: Event<Return>) => void,
	deps: React.DependencyList = [],
) => {
	useEffect(() => {
		const eventHandler = listen<Return>(eventName, callback)

		return () => {
			eventHandler.then((unlisten) => unlisten())
		}
	}, [eventName, callback, ...deps])
}

export const useOnce = <Return>(
	eventName: string,
	callback: (event: Event<Return>) => void,
	deps: React.DependencyList = [],
) => {
	useEffect(() => {
		const eventHandler = once<Return>(eventName, callback)

		return () => {
			eventHandler.then((unlisten) => unlisten())
		}
	}, [eventName, callback, ...deps])
}

// 2. Возвращаем функцию для ручного вызова вместо автоматического эффекта
export const useEmit = () => {
	return useCallback((eventName: string, payload?: any) => {
		return emit(eventName, payload)
	}, [])
}

export const useEmitTo = () => {
	return useCallback((target: string, eventName: string, payload?: any) => {
		return emitTo(target, eventName, payload)
	}, [])
}

// 3. Добавляем хук для обработки ошибок
export const useListenSafe = <Return>(
	eventName: string,
	callback: (event: Event<Return>) => void,
	onError?: (error: Error) => void,
	deps: React.DependencyList = [],
) => {
	useEffect(() => {
		const setupListener = async () => {
			try {
				const unlisten = await listen<Return>(eventName, callback)
				return unlisten
			} catch (error) {
				onError?.(error as Error)
				return null
			}
		}

		let unlisten: (() => void) | null = null
		setupListener().then((unlistenFn) => {
			unlisten = unlistenFn
		})

		return () => {
			unlisten?.()
		}
	}, [eventName, callback, onError, ...deps])
}

// 4. Добавляем хук для состояния событий
export const useEventState = <T>(eventName: string, initialValue: T) => {
	const [state, setState] = useState<T>(initialValue)

	useListen<T>(eventName, (event) => {
		setState(event.payload)
	})

	const emitState = useCallback(
		(newValue: T) => {
			setState(newValue)
			emit(eventName, newValue)
		},
		[eventName],
	)

	return [state, emitState] as const
}

// const onEvent = new Channel<DownloadEvent>();
// onEvent.onmessage = (message) => {
//   console.log(`got download event ${message.event}`);
// };

// await invoke('download', {
//   url: 'https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-schema-generator/schemas/config.schema.json',
//   onEvent,
// });

// 5. Добавляем хук для работы с каналами
export const useChannel = <T = any>(
	onMessage?: (message: T) => void,
	onError?: (error: Error) => void,
) => {
	const [channel, setChannel] = useState<Channel<T> | null>(null)
	const [isReady, setIsReady] = useState(false)

	useEffect(() => {
		const newChannel = new Channel<T>()

		// Устанавливаем обработчик сообщений
		if (onMessage) {
			newChannel.onmessage = (message: T) => {
				try {
					onMessage(message)
				} catch (error) {
					onError?.(error as Error)
				}
			}
		}

		setChannel(newChannel)
		setIsReady(true)

		return () => {
			// Очищаем канал при размонтировании
			setChannel(null)
			setIsReady(false)
		}
	}, [onMessage, onError])

	const updateHandler = useCallback(
		(newOnMessage?: (message: T) => void, newOnError?: (error: Error) => void) => {
			if (channel && newOnMessage) {
				channel.onmessage = (message: T) => {
					try {
						newOnMessage(message)
					} catch (error) {
						newOnError?.(error as Error)
					}
				}
			}
		},
		[channel],
	)

	return {
		channel,
		isReady,
		updateHandler,
	} as const
}

// 6. Хук для создания канала с состоянием
export const useChannelState = <T = any>(initialValue?: T) => {
	const [state, setState] = useState<T | undefined>(initialValue)
	const [error, setError] = useState<Error | null>(null)

	const { channel, isReady } = useChannel<T>(
		(message) => {
			setState(message)
			setError(null)
		},
		(err) => setError(err),
	)

	const clearError = useCallback(() => setError(null), [])

	return {
		channel,
		state,
		error,
		isReady,
		clearError,
	} as const
}
