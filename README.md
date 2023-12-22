![finance-demo](https://github.com/Lmedmo/finance-demo/assets/102483463/18c42d36-a49a-486b-b671-a94a6f24a1e9)

# Overview
This is a demo of a personal finance utility that I have been developing using SvelteKit, Tauri, and SQLite. The application will run on all platforms; macOS, Windows, Linux, iOS, Android, and the Web. The app comes with the full power of a relational database embedded directly into the application binary, unlocking nearly limitless possibilities regarding the variety of metrics that you can collect on your spending information, and because your data is stored locally, users will never have to worry about the security of their information being passed over a network. The cherry on top though it that with all the embedded features and custom UI assets, the app weighs in (currently) at just over 17 MB. Some of the key features and things you can do with Finance include:
- Custom window UI -- Consistent across all platforms.
- Calculate budget category spending totals
- Add Transactions quickly and see updated balances, trends, averages, and totals 
- Import bank statements and Finance will automatically detect merchants, depositors, transfers, and their associated categories/aliases
- Create and share custom budget plans
- View, edit, sort, filter and perform custom calculations on account records from any of your accounts.
- Create notifications to alert you of due dates, subscription renewal dates, and more.
- View trends and monitor your adherence to your goals and budgets.
- Light and Dark Modes
- Customizeable Fonts
- And more…

# Architecture
This app establishes a foundation upon which it is possible to rapidly build powerful, lightweight, and secure, cross-platform applications that are also highly customizable and user friendly. Components are designed to be generic and reusable, thus providing the architecture a flexibility to adapt for different use cases by simply changing a few variables, and defining a the necessary DB schema; for most purposes that’s all it would take.

## User Interface | *Frontend*

**`Routes`**: Defines each page of the app (ex. from demo: ‘Planning’, ‘Activity’, etc.). Each route or Page has 1 or more subdirectories which define Views.

**`Layouts`**: Formatting templates for UI and Content.

**`Window`**: Application UI: Titlebar → Ribbon → Explorer.

**`Components`**: Tables, Charts, Widgets, Buttons, Headers, etc.

**`Config`**: Modules for creating, retrieving and manipulating the values and types that define the behavior of the pages, views, explorer menus, toolbars, tables, forms, and the overall state of the application.

**`Forms`**: Input templates for the various data the app collects (transactions, accounts, budgets, etc.).

**`Models`**: Object datatypes (Interfaces) for the various database records.

**`Actions`**: Handlers for adding, deleting, importing, and editing records and performing calculations.

**`Controllers`**: Modules for interfacing with the Rust backend and database, organized by primary record type.

## Under the Hood | *Backend*

**`Main`**: Multi-threaded, asynchronous.

**`DB`**: Embedded, initialized and saved locally.

**`Models`**: Serialized types → Records, Users, Settings.

**`Commands`**: Backend Model Controller functions for reading, manipulating, comparing, and writing data to the store.

**`Controllers`**: Interface between frontend and backend.

**`Schema`** / **`Migrations`**: Define and redefine DB structure.

**`Utilities`**: Access native functions (files, command-line, etc.).

## Database | *Backend*
![E6F35E3B-F663-40AA-AB20-CF7E7F40BB93_1_201_a](https://github.com/Lmedmo/finance-demo/assets/102483463/ca4c7cb4-1fbb-4b9b-9b77-1a6fb7216bc3)

