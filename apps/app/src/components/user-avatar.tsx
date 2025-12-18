"use client";

import { createClient } from "@rust-v1/supabase/client";
import { Button } from "@rust-v1/ui/button";
import { Icons } from "@rust-v1/ui/icons";
import { useRouter } from "next/navigation";
import { useState } from "react";

interface UserAvatarProps {
  email?: string | null;
  name?: string | null;
}

export function UserAvatar({ email, name }: UserAvatarProps) {
  const [isOpen, setIsOpen] = useState(false);
  const supabase = createClient();
  const router = useRouter();

  const displayName = name || email?.split("@")[0] || "User";
  const initial = displayName.charAt(0).toUpperCase();

  const handleSignOut = async () => {
    await supabase.auth.signOut();
    router.push("/login");
  };

  return (
    <div className="relative">
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-3 p-2 rounded-xl hover:bg-white/5 transition-colors"
      >
        <div className="size-9 rounded-full bg-gradient-to-br from-purple-500 to-blue-500 flex items-center justify-center text-white font-semibold text-sm shadow-lg">
          {initial}
        </div>
        <div className="hidden sm:block text-left">
          <p className="text-sm font-medium text-white">{displayName}</p>
          {email && <p className="text-xs text-slate-400">{email}</p>}
        </div>
        <Icons.ChevronDown className={`size-4 text-slate-400 transition-transform ${isOpen ? "rotate-180" : ""}`} />
      </button>

      {isOpen && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-40"
            onClick={() => setIsOpen(false)}
          />

          {/* Dropdown */}
          <div className="absolute right-0 top-full mt-2 w-56 rounded-xl bg-slate-900 border border-white/10 shadow-xl z-50 overflow-hidden">
            <div className="p-3 border-b border-white/10">
              <p className="text-sm font-medium text-white">{displayName}</p>
              {email && <p className="text-xs text-slate-400 truncate">{email}</p>}
            </div>
            <div className="p-2">
              <Button
                onClick={handleSignOut}
                variant="ghost"
                className="w-full justify-start gap-2 text-slate-400 hover:text-white hover:bg-white/5"
              >
                <Icons.SignOut className="size-4" />
                Sign out
              </Button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
