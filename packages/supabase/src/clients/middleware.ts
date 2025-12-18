import { createServerClient } from "@supabase/ssr";
import type { ResponseCookie } from "next/dist/compiled/@edge-runtime/cookies";
import type { NextRequest, NextResponse } from "next/server";

interface CookieToSet {
  name: string;
  value: string;
  options?: Partial<ResponseCookie>;
}

export const updateSession = async (
  request: NextRequest,
  response: NextResponse,
) => {
  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        getAll() {
          return request.cookies.getAll();
        },
        setAll(cookiesToSet: CookieToSet[]) {
          for (const { name, value } of cookiesToSet) {
            request.cookies.set(name, value);
          }

          for (const { name, value, options } of cookiesToSet) {
            response.cookies.set(name, value, options);
          }
        },
      },
    },
  );

  // This is to ensure the session is updated
  const {
    data: { user },
  } = await supabase.auth.getUser();

  return { response, user };
};
