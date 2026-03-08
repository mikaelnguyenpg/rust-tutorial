**Bắt đầu với Git**
1. Mục tiêu
	- Nắm được các lệnh cơ bản làm việc với git command 
	- Tự đẩy code vào repository 
	- Sử dụng git trong các thao tác code theo yêu cầu.
2. Nội dung
	1.khái niệm git
	2. cài đặt và cấu hình
	3. các lệnh cơ bản


###
**2.1 Khái niệm Git**
	- Là hệ thống quản lý phiên bản phân tán (Distributed Version Control System).
	- Mục tiêu của git là hỗ trợ lập trình viên xử lý việc code khi có những phiên bản khác nhau, các nhóm (team coding) độc lập khác nhau trên 1 Dự án mã nguồn, git lưu trữ tất cả các tệp tin, lịch sử thay đổi, lập trình viên có thể theo dõi tình trạng cũng như truy vết
**2.2 Cài đặt và cấu hình git**
	**a. Cài đặt**
		**- Windows**
			- Tải và cài đặt `https://github.com/git-for-windows/git/releases/download/v2.53.0.windows.1/Git-2.53.0-64-bit.exe`
			- Trong môi trường Powershell 
			`winget install --id Git.Git -e --source winget`
(Tham khảo:https://git-scm.com/install/windows)
		**- MacOS**
	- sử dụng một trong các command sau trong terminal
		 `brew install git`
		 hoặc
		 `sudo port install git`
		 hoặc
		 `xcode-select --install`
		 **- Linux OS**
		- Debian/Ubuntu
			- `apt-get install git`
		- Fedora/redhat
			- `dnf install git`
		(Tham khảo:https://git-scm.com/install/linux)

**b. Cấu hình**

Việc đầu tiên bạn nên làm khi cấu hình Git là chỉ định tên tài khoản và địa chỉ e-mail. Điều này rất quan trọng vì Git sẽ sử dụng chúng cho mỗi lần commit, những thông tin này được gắn bất di bất dịch vào các commit:
- `git config --global user.name "John Doe"`
- `git config --global user.email johndoe@example.com`

- Check thông tin cấu hình
	- `git config --list`

**3. các lệnh cơ bản**
	**- Tạo folder trên local**
`git mkdir folder_name` - Tạo repository trong hệ thống local.
	**- Khởi tạo** 
`git init` 
	**- Thêm các tệp**
 
`$ git add .`(Chú ý dấu chấm)
Hoặc:
`git add --all `
`git add index.html` (có thể chỉ định trực tiếp tên tệp cần add)

--> Add những thay đổi (bạn đã tạo mới hoặc chỉnh sửa) để thực hiện commit
	**- Git commit:**
	`git commit -m "Thông điệp của bạn`"

Git commit: Ghi lại các thay đổi vào kho lưu trữ. (Cần thêm các thông điệp rõ ràng vào mỗi mục commit)
	**- Git push:**
	
`git push -u origin branch_name -` Push (đẩy) branch vào remote repository.
`git push `- Push (đẩy) tất cả mọi thay đổi (đã commit) lên remote repository.
`git push -d origin branch_name` - Xóa một branch trên remote repository.
	**- Git clone:**

` git clone <url> `(Địa chỉ dự án bạn muốn Clone) - Clone dự án có sẵn trên GitHub.
*(Mọi người tham khảo thêm tài liệu git command để nắm, hoặc có thể search git cheetsheet để nắm thêm các lệnh khác)*

**4. Làm việc với github và git command**
- Đăng ký tài khoản github					`https://github.com/signup`
 - sau khi đăng nhập
 	truy cập `https://github.com/new` để tạo repository
- sau khi tạo Repo xong thì github hướng dẫn bạn 7 command cơ bản khi dùng git (chỉ cần như vậy là đã hoàn toàn đẩy được code lên)

- trên thực tế sẽ có các brand khác nhau trong một repo, Dev cần check brand `git branch`  để xác định đúng brand mà mình đang làm việc.
