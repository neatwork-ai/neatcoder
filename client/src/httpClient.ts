import fetch from "node-fetch";
import { getOrSetApiKey } from "./utils";
const EventSource = require("eventsource");

export async function makeRequest(body: string): Promise<object> {
  const apiKey = getOrSetApiKey();

  try {
    const response = await fetch("https://api.openai.com/v1/chat/completions", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${apiKey}`,
      },
      body,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return (await response.json()) as object;
  } catch (error) {
    console.error("Error making request:", error);
    throw error;
  }
}

export function streamRequest() {
  const apiKey = getOrSetApiKey();

  try {
    const url = "https://api.openai.com/v1/chat/completions";
    const es = new EventSource(url, {
      headers: {
        Authorization: `Bearer ${apiKey}`,
      },
    });

    es.onmessage = function (event: MessageEvent) {
      console.log("Message:", event.data);
    };

    es.onerror = function (error: Event) {
      console.error("EventSource failed:", error);
      es.close();
    };
  } catch (error) {
    console.error("Error during streaming request:", error);
  }
}
