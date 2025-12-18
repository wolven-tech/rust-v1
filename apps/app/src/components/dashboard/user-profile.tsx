"use client";

import { Icons } from "@rust-v1/ui/icons";
import { useEffect, useState } from "react";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";

interface User {
  id: string;
  name: string;
  email: string;
}

export function UserProfile() {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchUser = async () => {
      try {
        const response = await fetch(`${API_URL}/api/users`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({}),
        });

        const data: User = await response.json();
        setUser(data);
      } catch (error) {
        console.error("Failed to fetch user:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchUser();
  }, []);

  if (loading) {
    return (
      <div className="space-y-4">
        <div className="flex items-center gap-3 mb-2">
          <div className="p-2 rounded-lg bg-green-500/20">
            <Icons.User className="size-5 text-green-400" />
          </div>
          <div>
            <h2 className="text-xl font-semibold text-white">User Profile</h2>
            <p className="text-sm text-slate-400">Your account details</p>
          </div>
        </div>
        <div className="flex items-center justify-center p-8">
          <Icons.Loader className="size-6 animate-spin text-slate-400" />
        </div>
      </div>
    );
  }

  if (!user) {
    return (
      <div className="space-y-4">
        <div className="flex items-center gap-3 mb-2">
          <div className="p-2 rounded-lg bg-green-500/20">
            <Icons.User className="size-5 text-green-400" />
          </div>
          <div>
            <h2 className="text-xl font-semibold text-white">User Profile</h2>
            <p className="text-sm text-slate-400">Your account details</p>
          </div>
        </div>
        <p className="text-slate-400">Failed to load user profile</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3 mb-2">
        <div className="p-2 rounded-lg bg-green-500/20">
          <Icons.User className="size-5 text-green-400" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-white">User Profile</h2>
          <p className="text-sm text-slate-400">Your account details</p>
        </div>
      </div>

      <div className="p-4 rounded-xl bg-white/5 border border-white/10 space-y-4">
        <div className="flex items-center gap-4">
          <div className="size-12 rounded-full bg-gradient-to-br from-green-400 to-blue-500 flex items-center justify-center text-white font-bold text-lg">
            {user.name.charAt(0).toUpperCase()}
          </div>
          <div>
            <p data-testid="user-name" className="font-medium text-white">
              {user.name}
            </p>
            <p data-testid="user-email" className="text-sm text-slate-400">
              {user.email}
            </p>
          </div>
        </div>

        <div className="h-px bg-white/10" />

        <div>
          <p className="text-xs text-slate-500 uppercase tracking-wider mb-1">
            User ID
          </p>
          <p
            data-testid="user-id"
            className="font-mono text-sm text-slate-300 bg-white/5 px-2 py-1 rounded inline-block"
          >
            {user.id}
          </p>
        </div>
      </div>
    </div>
  );
}
