# 語言名稱
locale-system = 使用系統語言
locale-english = 英文（美國）
locale-traditional-chinese = 繁體中文（臺灣）

# 共用操作與狀態
action-apply = 套用
action-back = 返回
action-cancel = 取消
action-close = 關閉
action-confirm = 確認
action-connect = 連線
action-continue = 繼續
action-create = 建立
action-delete = 刪除
action-dismiss = 略過
action-done = 完成
action-edit = 編輯
action-install = 安裝
action-open = 開啟
action-refresh = 重新整理
action-remove = 移除
action-reset = 重設
action-save = 儲存
action-send = 傳送
action-sign-in = 登入
action-sign-out = 登出
action-update = 更新
state-disabled = 已停用
state-enabled = 已啟用
state-loading = 載入中…
state-none = 無

# 基礎與測試訊息
language = 語言
language-description = 選擇 Smudgy 介面使用的語言。伺服器傳來的文字不會翻譯。
welcome-name = 歡迎，{ $name }。

# 設定視窗導覽
nav-account = 帳戶
nav-preferences = 偏好設定
nav-security = 安全性
nav-friends = 好友
nav-licenses = 授權資訊

# 帳戶與驗證
account-title = 帳戶
account-checking = 正在確認帳戶狀態…
account-verified = 已驗證
account-not-verified = 尚未驗證
account-signed-in-as = 已以 { $handle } 登入
account-signed-in-no-nickname = 已登入（尚未設定暱稱）
account-signed-in = 已登入。
account-verify-description = 請驗證電子郵件，才能使用好友、分享與同步等雲端功能。
account-code-placeholder = 電子郵件中的驗證碼
account-email-placeholder = 電子郵件
account-verify = 驗證
account-email-code = 寄送驗證碼
account-recheck = 重新確認
account-sign-out-everywhere = 同時登出所有裝置
account-choose-nickname = 選擇暱稱
account-nickname-description = 這是你的公開名稱；其他人會用它找到你並加你為好友。
account-nickname-placeholder = 暱稱（3–24 個英文字母、數字、- 或 _）
account-claim-handle = 使用此暱稱
account-nickname-change-warning = 變更暱稱後，其他人尋找你的方式也會改變。
account-change-nickname = 變更暱稱
account-sign-in-description = 我們會寄送一次性驗證碼，不需要密碼。第一次使用 Smudgy？輸入電子郵件後便會自動建立帳戶。
account-have-code = 我已有驗證碼
account-check-email = 查看電子郵件
account-code-sent = 驗證碼已寄到 { $email }。請貼在下方；驗證碼會在 15 分鐘後失效。
account-resend-code = 重新寄送
account-error-invalid-email = 請輸入有效的電子郵件地址。
account-busy-emailing-code = 正在寄送驗證碼…
account-error-paste-code = 請貼上電子郵件中的驗證碼。
account-error-email-for-code = 請先輸入電子郵件，才能確認驗證碼所屬的帳戶。
account-busy-verifying = 正在驗證…
account-notice-signed-in-needs-nickname = 登入成功！請選擇一個暱稱。
account-notice-signed-in-ready = 登入成功，一切就緒。
account-error-invalid-code = 驗證碼無效或已過期，請重新取得。
account-error-email-for-resend = 請先在表單中輸入要接收驗證碼的電子郵件。
account-busy-sending = 正在寄送…
account-notice-code-resent = 若該地址已有帳戶，新的驗證碼已寄出，並會取代先前的驗證碼。
account-busy-saving-nickname = 正在儲存暱稱…
account-notice-nickname-changed = 你現在的暱稱是 { $handle }。
account-notice-signed-out = 已登出。
account-error-nickname-empty = 請輸入暱稱。
account-error-nickname-format = 暱稱必須是 3–24 個英文字母、數字、'-' 或 '_'。

# 偏好設定
preferences-title = 偏好設定
preferences-appearance = 外觀
preferences-terminal-font = 終端機字型
preferences-font-size = 字型大小
preferences-press-enter = 按 Enter 套用。
preferences-line-length = 每行長度
preferences-wrap-window = 依視窗換行
preferences-line-length-help = 每行在換行前可顯示的字元上限；留空則依視窗寬度換行。按 Enter 套用。
preferences-theme = 主題
preferences-scrollback = 回捲行數
preferences-scrollback-help = 每個連線保留的行數。按 Enter 套用。
preferences-hide-pane-headers = 只在主選單展開時顯示面板標題列
preferences-hide-pane-headers-help = 工作階段與窗格的標題列只會在視窗工具列展開時顯示；標題列也是拖曳重排窗格的把手，分隔線仍可隨時調整大小。腳本可指定某個窗格永遠顯示標題列。
preferences-input = 輸入
preferences-command-separator = 指令分隔符號
preferences-command-separator-help = 將同一輸入行拆成多個指令。
preferences-raw-prefix = 原樣傳送前綴
preferences-raw-prefix-help = 以前綴開頭的內容會移除前綴後原樣送出，略過別名與上方的指令分隔規則。
preferences-command-input = 指令輸入框
preferences-command-input-help = 按下 Enter 後，輸入框與其中的文字要如何處理
preferences-input-select-clear = 送出後全選，失去焦點時清除
preferences-input-select = 送出後全選
preferences-input-clear = 送出後清除
preferences-logging = 記錄
preferences-logging-plain = 寫入工作階段記錄（純文字）
preferences-logging-raw = 同時寫入原始記錄（包含 ANSI 色彩碼）
preferences-logging-raw-help = 原始記錄會從下一次連線開始寫入。
preferences-advanced = 進階
preferences-advanced-scripting = 啟用進階腳本功能
preferences-advanced-scripting-help = 解鎖「移除沙箱」（讓已安裝套件如同自己撰寫的腳本般取得完整存取權）及「自動化」視窗中的腳本檢查器。變更後請重新開啟「自動化」。
preferences-updates = 更新
preferences-auto-updates = 自動檢查更新
preferences-invalid-value = 無效的值

# 主題編輯器
theme-adjust = 調整
theme-colors = 色彩
theme-override = 覆寫：{ $slot }
theme-clear-override = 清除覆寫
theme-reset-colors = 重設所有色彩
theme-background = 背景
theme-brightness = 亮度
theme-contrast = 對比
theme-saturation = 飽和度
theme-adjust-help = 「背景」只移動表面色彩；「對比」會拉開文字與背景的差距。
theme-reset-adjustments = 重設調整值
theme-preview = 敏捷的棕狐躍過懶狗

# 安全性與工作階段
security-title = 安全性
security-api-keys = API 金鑰
security-new-key = 新的 API 金鑰只會顯示這一次，請立即複製：
security-key-copied = 完成，我已複製
security-no-api-keys = 沒有 API 金鑰。
security-key-created = 建立於 { $date }
security-last-used = 上次使用於 { $date }
security-never-used = 從未使用
security-revoke = 撤銷
security-create-key = 建立 API 金鑰
security-create-key-help = 必須先登入工作階段才能建立金鑰；金鑰只會在建立時顯示一次。
security-create-key-login-error = 必須使用登入工作階段才能建立 API 金鑰，僅提供 API 金鑰並不足夠。
security-sessions = 工作階段
security-no-sessions = 沒有使用中的工作階段。
security-session-created = 建立於 { $date }
security-session-expires = 到期日 { $date }
security-session-unused = 尚未使用
security-revoke-current-warning = 撤銷目前使用的工作階段會使你登出。

# 好友與授權
friends-title = 好友
friends-verify-email = 驗證電子郵件後即可新增好友及分享地圖
friends-go-account = 前往帳戶

# 用戶端驗證標籤
validation-invalid-value = 無效的值

# 伺服器文字編碼
encoding-label = 伺服器文字編碼
encoding-utf8 = UTF-8（建議）
encoding-big5 = Big5（繁體中文舊式伺服器）
encoding-help = 只有在這個 MUD 傳送舊式繁體中文字碼時才選擇 Big5。Telnet、GMCP 與 MSDP 控制資料不會轉碼。
connection-connecting = 正在連線至 { $address }…
connection-connected = 已連線。
connection-disconnected = 已中斷連線。
connection-lost = 連線已中斷
connection-failed = 連線失敗
connection-decode-warning = 部分伺服器文字不是有效的 { $encoding }，已以替代字元顯示。
connection-encode-warning = 部分輸入字元無法以 { $encoding } 表示，已改送數字字元參照。

# 伺服器與設定檔管理
servers-title = 伺服器
servers-loading = 正在載入伺服器…
servers-empty = 尚未新增伺服器。
servers-new = + 新增伺服器
servers-get-started = 新增伺服器以開始使用
servers-get-started-help = 先新增伺服器，再建立用來登入的設定檔。
servers-add-first = 新增第一個伺服器
servers-select = 從清單中選擇伺服器。
server-name = 名稱
server-name-placeholder = 例如 ArcticMud
server-host = 主機
server-port = 連接埠
server-port-help = 通常是 23 或 4000。
server-add = 新增伺服器
server-edit = 編輯伺服器
server-delete = 刪除伺服器
server-save-add-profile = 儲存並新增設定檔
server-confirm-delete = 確定要刪除伺服器「{ $name }」嗎？此操作無法復原。
server-confirm-delete-action = 是，刪除此伺服器
server-error-port = 連接埠無效，必須介於 1 到 65535。
server-error-name-empty = 伺服器名稱不能留空。
server-error-config = 設定錯誤：{ $error }
server-error-create = 無法建立伺服器：{ $error }
server-error-update = 無法更新伺服器：{ $error }
server-error-delete = 無法刪除伺服器：{ $error }
server-error-action-delete = 發生非預期錯誤：確認刪除時不能送出表單。
server-error-action-missing = 發生非預期錯誤：目前沒有可執行的操作。
server-edit-short = 編輯
server-details-missing = 找不到伺服器詳細資料。

profiles-title = 設定檔
profiles-saved-help = 此伺服器已儲存的登入方式。
profiles-load-error = 無法載入此伺服器的設定檔。
profiles-error-load = 載入伺服器「{ $server }」的設定檔時發生錯誤：{ $error }
profiles-new = + 新增設定檔
profiles-empty = 新增設定檔後即可連線
profile-name = 設定檔名稱
profile-name-placeholder = 甘道夫
profile-description = 說明（選填）
profile-description-placeholder = 法師
profile-add = 新增設定檔
profile-create = 建立設定檔
profile-edit = 編輯設定檔
profile-delete = 刪除設定檔
profile-confirm-delete = 確定要刪除設定檔「{ $name }」嗎？
profile-confirm-delete-action = 是，刪除此設定檔
profile-connect = 連線
profile-offline = 離線開啟
profile-on-connect = 連線後傳送（選填）
profile-on-connect-placeholder = Gandalf\n$PASSWORD
profile-plaintext-notice = 自動登入文字會以純文字儲存在此裝置。請使用 $PASSWORD 安全插入密碼。
profile-keychain-notice = 自動登入文字會以純文字儲存；$PASSWORD 的值則保存在作業系統鑰匙圈。
profile-password-saved = 密碼已儲存
profile-password-change = 變更
profile-password-clear = 清除
profile-password-label = $PASSWORD 使用的密碼
profile-password-placeholder = 密碼
profile-error-no-server = 錯誤：尚未選擇伺服器。
profile-error-password-required = 請輸入 $PASSWORD 使用的密碼，或從自動登入文字移除 $PASSWORD。
profile-error-name-empty = 設定檔名稱不能留空。
profile-error-config = 設定錯誤：{ $error }
profile-error-action-delete = 發生非預期錯誤：確認刪除時不能送出表單。
profile-error-action-missing = 發生非預期錯誤：目前沒有可執行的操作。
profile-error-delete-no-server = 錯誤：確認刪除時尚未選擇伺服器。
profile-error-password-clear = 無法清除已儲存的密碼：{ $error }
profile-error-create = 無法建立設定檔：{ $error }
profile-error-update = 無法更新設定檔：{ $error }
profile-error-delete = 無法刪除設定檔：{ $error }
