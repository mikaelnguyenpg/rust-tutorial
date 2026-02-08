1. Box/Rc/Arc

- Dùng để lưu value vào bộ nhớ Heap
- Lý do:
  1. Khai báo biến dyn
  2. Recursive type

2. Cell, RefCell

3. Weak // hoan
   Rc/Arc => Rc/Arc:downgrade -> Weak<T>
   Weak<T> -> upgrade -> T

4. Mutex trong threading

- Process:
  - main thread
  - spawn: nhieu thread
    => data races

// NEXT

BE: rust -> Clean Architecture
FE: Web + NEXT.js / web _ ios _ android = Rust

TanStack

---

Bat dau tim hieu
--->

---

AI: hoan thien hon ve phan code
GC
Golang, C#, Java
Python + Rust (ownership + borrowing)
Typescript + Rust

Tap trung:
==> Logic, tu duy lap trinh => hoan thanh yeu cau ve business
A -> B
--> Test: QA --> automated test

=> Kien truc du an: TA/SA

- HTTP server: security, auth/autho..., ddos...
- Database: ... trait/interface
- 3th services: ...

=> Clean architecture

===
FE: https://dioxuslabs.com/learn/0.7/
web _ ios _ android = Rust

React Native: JS runtime (Expo)
Flutter

dioxus: hybrid -> Webview (v0.7)
tauri -> desktop (electron)

Web:

- React
- Angular
