export const messageAttachmentQueryKeys = {
  all: ["message-attachments"] as const,
  byMessage: (messageId: string) =>
    [...messageAttachmentQueryKeys.all, "message", messageId] as const,
  file: (messageId: string, fileId: string) =>
    [...messageAttachmentQueryKeys.byMessage(messageId), "file", fileId] as const,
};
