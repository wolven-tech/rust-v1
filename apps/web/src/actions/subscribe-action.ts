"use server";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";

interface SubscribeResponse {
  success: boolean;
  message?: string;
  error?: string;
}

export async function subscribeAction(
  formData: FormData,
  userGroup: string,
): Promise<SubscribeResponse> {
  const email = formData.get("email") as string;

  try {
    const response = await fetch(`${API_URL}/api/subscribe`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email,
        user_group: userGroup,
      }),
    });

    const data = await response.json();

    if (!response.ok) {
      return {
        success: false,
        error: data.error || "Failed to subscribe",
      };
    }

    return {
      success: true,
      message: data.message,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : "Failed to subscribe",
    };
  }
}
