"use client";

import { Button } from "@rust-v1/ui/button";
import { Icons } from "@rust-v1/ui/icons";
import { Input } from "@rust-v1/ui/input";
import { useState } from "react";
import { useCalculateShipping } from "@/hooks/api";
import type { ShippingCost } from "@/domain/api";

export function ShippingCalculator() {
  const [weight, setWeight] = useState("");
  const [result, setResult] = useState<ShippingCost | null>(null);
  const [validationError, setValidationError] = useState<string | null>(null);

  const shippingMutation = useCalculateShipping();

  const handleCalculate = async () => {
    const weightNum = parseFloat(weight);

    if (isNaN(weightNum) || weightNum <= 0) {
      setValidationError("Please enter a valid positive weight");
      setResult(null);
      return;
    }

    setValidationError(null);

    shippingMutation.mutate(
      { weight: weightNum },
      {
        onSuccess: (data: ShippingCost) => {
          setResult(data);
        },
        onError: () => {
          setResult(null);
        },
      },
    );
  };

  const error = validationError || shippingMutation.error?.message;
  const isLoading = shippingMutation.isPending;

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3 mb-2">
        <div className="p-2 rounded-lg bg-blue-500/20">
          <Icons.Calculator className="size-5 text-blue-400" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-white">Shipping Calculator</h2>
          <p className="text-sm text-slate-400">Estimate delivery costs</p>
        </div>
      </div>

      <div className="flex gap-2">
        <Input
          type="number"
          placeholder="Weight (kg)"
          value={weight}
          onChange={(e) => setWeight(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleCalculate()}
          className="flex-1 bg-white/5 border-white/10 text-white placeholder:text-slate-500"
        />
        <Button onClick={handleCalculate} disabled={isLoading}>
          {isLoading ? (
            <Icons.Loader className="size-4 animate-spin" />
          ) : (
            "Calculate"
          )}
        </Button>
      </div>

      {error && (
        <div
          data-testid="shipping-error"
          className="p-3 bg-red-500/10 text-red-400 rounded-xl border border-red-500/20"
        >
          {error}
        </div>
      )}

      {result && (
        <div
          data-testid="shipping-result"
          className="p-4 rounded-xl bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-white/10 space-y-3"
        >
          <div className="flex items-center justify-between">
            <span className="text-slate-400">Weight</span>
            <span className="font-medium text-white">{result.weight} kg</span>
          </div>
          <div className="h-px bg-white/10" />
          <div className="flex items-center justify-between">
            <span className="text-slate-400">Shipping Cost</span>
            <span data-testid="shipping-cost" className="font-bold text-2xl text-blue-400">
              ${result.cost.toFixed(2)}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}
