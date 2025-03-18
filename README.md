### REngine? REngine!

REngine - учебный проект, разрабатываемый в рамках ознакомления с языком Rust.

### Что такое REngine?

REngine - приложение для взаимодействия с устройствами на ОС Windows, для построения GUI и управления процессами.

`На текущий момент в проекте реализована возможность управления процессами с помощью CLI`

#### Из чего состоит REngine?

- cli - инструмент командной строки, для получения информации о системе и процессах в реальном времени
- core - основной модуль приложения
- machine - микро-обертка над библиотекой `sysinfo` для сбора информации о системе
- sdl3 - обертка над библиотекой `SDL3.dll`
- win - модуль взаимодействия с ОС Windows через обращение к kernel32.dll

### Немного об SDL3

*\*ознакомиться с SDL можно здесь:* [SDL3 Wiki](https://wiki.libsdl.org/SDL3/FrontPage)

На данный момент реализованы следующие типы и методы:

- Типы:
	- `SDL_Init(u32)` -> `bool`
	- `SDL_Quit()` -> `void`
	- `SDL_GetError()` -> `*const c_char`
	- `SDL_CreateWindow(*const i8, u32, u32, u32)` -> `*mut c_void`
	- `SDL_DestroyWindow(*mut c_void)` -> `void`
	- `SDL_Delay(u32)` -> `void`
	- `SDL_PollEvent(&mut SDL_Event)` -> `bool`

- Методы
	- `new() -> SDL3`
	- `run(&mut self) -> void`
	- `sdl3_init(&mut self, flags: u32) -> void` инициализация системы `SDL3`
	- `sdl3_quit(&mut self) -> void` завершение системы `SDL3`
	- `sdl3_create_window(&mut self, title: &str, w: u32, h: u32, flags: u32) -> *mut c_void` создание окна `SDL3`
	- `sdl3_destroy_window(&mut self, window: *mut c_void) -> void` удаление окна `SDL3`
	- `sdl3_poll_event(&mut self, ms: u32) -> void` поток обработки событий `SDL3`
	- `sdl3_get_error(&mut self)` -> `String` получение ошибки системы `SDL3`

- Структуры
	- `SDL3` -> управляющая структура системы `SDL3`

#### Для чего?

Т.к. REngine является лишь учебным проектом, то в рамках идеи в первую очередь поставлена задача реализации механизмов взаимодействия с ОС Windows и функций `Tasmgr.exe`

### И конечно же... Почему?

Просто ради практики ;)

### Special thanks

- https://doc.rust-lang.org/book/title-page.html
- https://os.phil-opp.com/
- https://habr.com/ru/companies/ruvds/articles/581042/
