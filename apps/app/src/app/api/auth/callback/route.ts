import {
  safeRedirectOrigin,
  safeRedirectPath,
} from "@/lib/safe-redirect";
import { createClient } from "@rust-v1/supabase/server";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const { searchParams, origin } = new URL(request.url);
  const code = searchParams.get("code");
  const next = safeRedirectPath(searchParams.get("next"));

  if (code) {
    const supabase = await createClient();
    const { error } = await supabase.auth.exchangeCodeForSession(code);
    if (!error) {
      const isLocalEnv = process.env.NODE_ENV === "development";

      if (isLocalEnv) {
        return NextResponse.redirect(`${origin}${next}`);
      }

      const redirectOrigin = safeRedirectOrigin(
        origin,
        request.headers.get("x-forwarded-host"),
      );

      return NextResponse.redirect(`${redirectOrigin}${next}`);
    }
  }

  return NextResponse.redirect(`${origin}?error=auth-code-error`);
}
