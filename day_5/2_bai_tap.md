1. Tự viết một đoạn chương trình bất kỳ
   Gợi ý:

- Sắp xếp thứ tự của một chuỗi n phần tử theo thứ tự tăng/giảm dần
- Xây dựng một B-tree

2. Hoàn thiện function describe_weak_reference
   Gợi ý:

- Weak được tạo từ Rc object, ví đụ:

```
  let rc = Rc::new(Teacher);
  let we = Rc::downgrade(rc); // -> Weak<Teacher>
```

Yêu cầu:

- Khởi tạo teacher A và B
- Gán B là đồng nghiệp (coworker) của A
- Gán A là đồng nghiệp (coworker) của B
- In ra giá trị của A
- drop A: mem::drop(a);
- In ra đồng nghiệp của B nếu có, nếu không có in ra B không có đồng nghiệp
