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
	- `SDL_GetError()` -> `*const i8`
	- `SDL_CreateWindow(*const i8, i32, i32, i32, i32, u32)` -> `*mut c_void`

- Методы
	- `load_sdl3()` -> `Result<Library, libloading::Error>` загрузка библиотеки SDL3
	- `sdl3_init(&mut self)` -> `bool` инициализация системы SDL3
	- `sdl3_quit(&mut self)` -> .. завершение системы SDL3
	- `sdl3_get_error(&mut self)` -> `*const i8` получение ошибки системы SDL3

- Структуры
	- `SDL3` -> управляющая структура системы SDL3

#### Для чего?

Т.к. REngine является лишь учебным проектом, то в рамках идеи в первую очередь поставлена задача реализации механизмов взаимодействия с ОС Windows и функций `Tasmgr.exe`

### И конечно же... Почему?

Просто ради практики ;)

### Special thanks

https://doc.rust-lang.org/book/title-page.html
https://os.phil-opp.com/
https://habr.com/ru/companies/ruvds/articles/581042/
