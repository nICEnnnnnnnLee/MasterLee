# Master Lee L大师

一个`tauri`实现的本地 HTTP + SNI代理, 能够智(☞智障的智)能查找可用的DNS over HTTPS服务, 为目标域名筛选最优IP。  

+ 针对保护列表的域名, 使用DoH查询的最优ip
+ 针对其它域名, 和不走代理表现一致

## 使用场景
适用于任何改host的场景。

☞ Q: 为什么找到IP之后不直接改host要用代理呢?  
☞ A: ![为什么 这个问题我也想问 我也不明白](why_not_host.gif)  

## 使用方法  
+ 点击`查找可用DoH`按钮, 根据[DNSCrypt](https://github.com/DNSCrypt/dnscrypt-resolvers)查找并筛选可用DoH服务
+ 点击`查找可用IP`按钮, 根据现有的DoH服务查找并筛选最优ip(需要等待上一步完成)
+ 点击`打开Proxy`
+ ps： 之后可以在不关闭Proxy的情况下, 更新DoH(可以不更新) 和 host ip

### HTTP 代理
和其它代理使用方法一致

### SNI 代理
举例说明，如果你想访问`https://github.com:443/`：
+ 程序监听端口设置为`443`并按照使用流程打开Proxy
+ 添加host记录`127.0.0.1 github.com`
+ 现在你可以访问该地址了
