let message = "";

export function setMessage(newMessage: string) {
  message = newMessage;
}

export function getMessage(): string {
  return message;
}