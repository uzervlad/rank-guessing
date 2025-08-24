type Reader = ReadableStreamDefaultReader<Uint8Array<ArrayBuffer>>;
type SSEMessageHandler = (message: any) => any;

export async function sseListen(reader: Reader, onMessage: SSEMessageHandler) {
  const decoder = new TextDecoder();

  let buffer = "";

  function processLines() {
    let lines = buffer.split("\n");
    buffer = lines.pop() || "";
    
    for (const line of lines) {
      if (line.trim().length > 0) {
        try {
          const message = JSON.parse(line);
          onMessage(message);
        } catch(err) {
          // idk yet
        }
      }
    }
  }

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    buffer += decoder.decode(value, { stream: true });
    
    processLines();
  }

  if (buffer.trim().length > 0) {
    processLines();
  }
}