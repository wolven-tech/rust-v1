"use client";

import { createClient } from "@rust-v1/supabase/client";
import { Button } from "@rust-v1/ui/button";
import { Icons } from "@rust-v1/ui/icons";
import { useRouter } from "next/navigation";

export function SignOut() {
  const supabase = createClient();
  const router = useRouter();

  const handleSignOut = async () => {
    await supabase.auth.signOut();
    router.push("/login");
  };

  return (
    <Button
      onClick={handleSignOut}
      variant="outline"
      className="font-mono gap-2 flex items-center"
    >
      <Icons.SignOut className="size-4" />
      <span>Sign out</span>
    </Button>
  );
}
