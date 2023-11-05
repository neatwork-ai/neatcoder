import { storeChat } from "../utils";

// TypeScript type for a function that returns a Promise<void>
type AsyncVoidFunction = () => Promise<void>;

export class ChatOperationQueue {
  private queue: AsyncVoidFunction[] = [];
  private processing: boolean = false;

  // Adds an operation to the queue
  public add(operation: AsyncVoidFunction) {
    this.queue.push(operation);
    this.processNext();
  }

  // Processes the next operation if the queue is not currently processing
  private async processNext() {
    if (this.processing || this.queue.length === 0) {
      return;
    }

    this.processing = true;
    const operation = this.queue.shift(); // Take the first operation from the queue

    if (operation) {
      try {
        await operation();
      } catch (error) {
        console.error("An error occurred during chat operation:", error);
        // Handle error, for example by retrying the operation or alerting the user
      }
      this.processing = false;
      this.processNext(); // Continue with the next operation
    }
  }
}
