import { GoogleSignin } from "@/components/google-signin";
import Image from "next/image";

export const metadata = {
  title: "Login",
};

export default function Page() {
  return (
    <div className="relative h-screen w-screen flex flex-col items-center justify-center overflow-hidden">
      {/* Magic gradient background */}
      <div className="absolute inset-0 bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950" />

      {/* Animated gradient orbs */}
      <div className="absolute top-1/4 -left-32 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl animate-pulse" />
      <div className="absolute bottom-1/4 -right-32 w-96 h-96 bg-blue-500/20 rounded-full blur-3xl animate-pulse delay-1000" />
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-indigo-500/10 rounded-full blur-3xl" />

      {/* Grid pattern overlay */}
      <div
        className="absolute inset-0 opacity-20"
        style={{
          backgroundImage: `linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
                           linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px)`,
          backgroundSize: "50px 50px",
        }}
      />

      {/* Content */}
      <div className="relative z-10 flex flex-col items-center justify-center space-y-8">
        {/* Logo with glow effect */}
        <div className="relative">
          <div className="absolute inset-0 bg-white/20 rounded-full blur-2xl scale-150" />
          <Image
            src="/logo.png"
            alt="Wolven Tech"
            width={160}
            height={160}
            quality={100}
            className="relative rounded-full ring-2 ring-white/10 shadow-2xl"
          />
        </div>

        {/* Title */}
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold text-white tracking-tight">
            Welcome Back
          </h1>
          <p className="text-slate-400 text-sm">
            Sign in to continue to your dashboard
          </p>
        </div>

        {/* Sign in button */}
        <div className="w-full max-w-xs">
          <GoogleSignin />
        </div>

        {/* Footer */}
        <p className="text-xs text-slate-500 mt-8">Powered by Wolven Tech</p>
      </div>
    </div>
  );
}
