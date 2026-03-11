# 21. Web Server

## 핵심 개념

### TCP 서버
- `TcpListener::bind()`: 포트 바인딩
- `.incoming()`: 연결 스트림
- `TcpStream`: 클라이언트 연결

### HTTP 프로토콜
- 요청: `GET / HTTP/1.1\r\n`
- 응답: `HTTP/1.1 200 OK\r\n\r\nBody`
- 헤더: Content-Length 등

### 멀티스레드
- **스레드 풀**: 고정된 수의 워커 스레드
- 채널로 작업 분배
- 우아한 종료(graceful shutdown): `Drop` 트레이트 구현

## 실행

```bash
rustc web_server.rs && ./web_server
```
