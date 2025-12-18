"use client";

import { Icons } from "@rust-v1/ui/icons";
import { useMetrics } from "@/hooks/api";

interface MetricCardProps {
  title: string;
  value: string | number;
  subtitle: string;
  icon: React.ReactNode;
  trend?: { value: number; label: string };
  color: "purple" | "blue" | "green" | "orange";
  loading?: boolean;
}

function MetricCard({ title, value, subtitle, icon, trend, color, loading }: MetricCardProps) {
  const colorClasses = {
    purple: "from-purple-500/20 to-purple-600/10 border-purple-500/20",
    blue: "from-blue-500/20 to-blue-600/10 border-blue-500/20",
    green: "from-green-500/20 to-green-600/10 border-green-500/20",
    orange: "from-orange-500/20 to-orange-600/10 border-orange-500/20",
  };

  const iconColorClasses = {
    purple: "bg-purple-500/20 text-purple-400",
    blue: "bg-blue-500/20 text-blue-400",
    green: "bg-green-500/20 text-green-400",
    orange: "bg-orange-500/20 text-orange-400",
  };

  return (
    <div className={`p-5 rounded-2xl bg-gradient-to-br ${colorClasses[color]} border backdrop-blur-sm`}>
      <div className="flex items-start justify-between">
        <div className="space-y-1">
          <p className="text-sm text-slate-400">{title}</p>
          {loading ? (
            <div className="h-9 w-16 bg-white/10 rounded animate-pulse" />
          ) : (
            <p className="text-3xl font-bold text-white">{value}</p>
          )}
          <p className="text-xs text-slate-500">{subtitle}</p>
        </div>
        <div className={`p-2.5 rounded-xl ${iconColorClasses[color]}`}>
          {icon}
        </div>
      </div>
      {trend && (
        <div className="mt-3 pt-3 border-t border-white/5">
          <span className={`text-xs ${trend.value >= 0 ? "text-green-400" : "text-red-400"}`}>
            {trend.value >= 0 ? "+" : ""}{trend.value}%
          </span>
          <span className="text-xs text-slate-500 ml-1">{trend.label}</span>
        </div>
      )}
    </div>
  );
}

export function MetricsCards() {
  const { data: metrics, isLoading } = useMetrics({
    refetchInterval: 3000, // Auto-refresh every 3 seconds
  });

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
      <MetricCard
        title="Total Products"
        value={metrics?.products ?? 0}
        subtitle="In catalog"
        icon={<Icons.Search className="size-5" />}
        color="purple"
        loading={isLoading}
      />
      <MetricCard
        title="Total Orders"
        value={metrics?.orders ?? 0}
        subtitle="Created via API"
        icon={<Icons.Plus className="size-5" />}
        color="blue"
        loading={isLoading}
      />
      <MetricCard
        title="Users"
        value={metrics?.users ?? 0}
        subtitle="Registered"
        icon={<Icons.User className="size-5" />}
        color="green"
        loading={isLoading}
      />
      <MetricCard
        title="API Calls"
        value={metrics?.api_calls ?? 0}
        subtitle="Total requests"
        icon={<Icons.Calculator className="size-5" />}
        color="orange"
        loading={isLoading}
      />
    </div>
  );
}
