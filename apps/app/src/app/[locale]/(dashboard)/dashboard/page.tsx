import { CreateOrder } from "@/components/dashboard/create-order";
import { MetricsCards } from "@/components/dashboard/metrics-cards";
import { ShippingCalculator } from "@/components/dashboard/shipping-calculator";
import { UserAvatar } from "@/components/user-avatar";
import { getUser } from "@rust-v1/supabase/queries";

export const metadata = {
  title: "Dashboard - API Demo",
};

export default async function DashboardPage() {
  const { data } = await getUser();
  const user = data?.user;

  return (
    <div className="relative min-h-screen">
      {/* Background */}
      <div className="fixed inset-0 bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 -z-10" />
      <div className="fixed top-0 right-0 w-[500px] h-[500px] bg-purple-500/10 rounded-full blur-3xl -z-10" />
      <div className="fixed bottom-0 left-0 w-[500px] h-[500px] bg-blue-500/10 rounded-full blur-3xl -z-10" />

      {/* Content */}
      <div className="relative z-10 p-4 md:p-8">
        <div className="max-w-6xl mx-auto space-y-8">
          {/* Top Bar */}
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-white/5 border border-white/10 text-xs text-slate-400">
              <span className="size-1.5 rounded-full bg-green-500 animate-pulse" />
              API Connected
            </div>
            <UserAvatar
              email={user?.email}
              name={user?.user_metadata?.full_name}
            />
          </div>

          {/* Header */}
          <header className="text-center space-y-3 py-4">
            <h1 className="text-3xl md:text-4xl font-bold text-white tracking-tight">
              Dashboard
            </h1>
            <p className="text-slate-400 max-w-xl mx-auto text-sm">
              Full-stack demo: Next.js + Rust API + Shared UI Components
            </p>
          </header>

          {/* Metrics Cards */}
          <MetricsCards />

          {/* Main Grid */}
          <div className="grid gap-6 lg:grid-cols-2">
            {/* Create Order */}
            <section className="p-6 rounded-2xl bg-gradient-to-br from-purple-500/5 to-blue-500/5 backdrop-blur-sm border border-white/10 hover:border-white/20 transition-colors">
              <CreateOrder />
            </section>

            {/* Shipping Calculator */}
            <section className="p-6 rounded-2xl bg-white/5 backdrop-blur-sm border border-white/10 hover:border-white/20 transition-colors">
              <ShippingCalculator />
            </section>
          </div>

          {/* Footer */}
          <footer className="text-center py-8 border-t border-white/10 space-y-2">
            <p className="text-xs text-slate-600">
              <a
                href="https://github.com/wolven-tech"
                target="_blank"
                rel="noopener noreferrer"
                className="hover:text-slate-400 transition-colors"
              >
                Wolven Tech
              </a>{" "}
              &middot; Next.js + Rust + AllFrame
            </p>
          </footer>
        </div>
      </div>
    </div>
  );
}
