# Init project

- cargo init
- cargo new <name>

# Run project

- cargo run

- cargo watch -x check -x test -x run

# Install parkage

- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- cargo install cargo-watch
  // test, build auto when save project

# Study

## Variable

- Immutable : if make mutable => let mut

## Shadowing

- Che giá trị của biến đã khởi tạo trước đó.

## Data types

Scalar Data

- let x = 100;
- Integer
- String
- Float
- Boolean

Compound Data

- Tuple
- Array

# Ownership, Borrowing, Reference

Dùng để dọn rác, quản lý bộ nhớ

- Đầu tiên là GC: Có ở ngôn ngữ bật cao: python, java -> chậm
- Ownership: Có thể quản lý bộ nhớ thủ công: c, c++, rust

Quy tắc:

- One variable one owner
- When owner out of block, value of owner will be drop
- Do not have 2 owner for 1 variable

Note: Không thể cùng tham chiếu đến 1 biến mutable!!!

- Wrong:
  let mut s = String::from("hello");
  let \_r1 = &mut s;
  let \_r2 = &mut s;
- True:
  let s = String::from("hello");
  let \_r1 = &s;
  let \_r2 = &s;

# Enum

# match

like switch case

# Option

- some
- none

# module

- mod <name> : như 1 file <name>.rs
- import mod => use <name>
- enum: chỉ cần public biến enum => pub enum
- fn: pub thì cần pub hết func trong mod

# Collection

- Được lưu ở trong heap

## vector

- init:
  vec![]
  Vec::new()
- hash map

# Generic type

# Trait

- Like abstract class in java

# lifetime
