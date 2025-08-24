let open = false;

export function toggleSubmissions() {
  open = !open;
}

export function submissionsOpen() {
  return open;
}