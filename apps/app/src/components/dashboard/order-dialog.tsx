"use client";

import { Button } from "@rust-v1/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@rust-v1/ui/dialog";
import { Icons } from "@rust-v1/ui/icons";
import { Input } from "@rust-v1/ui/input";
import { useState } from "react";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";

interface OrderResult {
  order_id: string;
  product: string;
  status: string;
}

export function OrderDialog() {
  const [open, setOpen] = useState(false);
  const [product, setProduct] = useState("");
  const [quantity, setQuantity] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<OrderResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async () => {
    if (!product.trim() || !quantity.trim()) {
      setError("Please fill in all fields");
      return;
    }

    const quantityNum = parseInt(quantity, 10);
    if (isNaN(quantityNum) || quantityNum <= 0) {
      setError("Please enter a valid quantity");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const response = await fetch(`${API_URL}/api/orders`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ product, quantity: quantityNum }),
      });

      if (!response.ok) {
        throw new Error("Order failed");
      }

      const data: OrderResult = await response.json();
      setResult(data);
      setOpen(false);
    } catch (err) {
      setError("Failed to create order");
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setProduct("");
    setQuantity("");
    setError(null);
  };

  return (
    <div className="space-y-4">
      <Dialog
        open={open}
        onOpenChange={(isOpen) => {
          setOpen(isOpen);
          if (!isOpen) resetForm();
        }}
      >
        <DialogTrigger asChild>
          <Button className="gap-2">
            <Icons.Plus className="size-4" />
            Create Order
          </Button>
        </DialogTrigger>
        <DialogContent className="bg-slate-900 border-white/10">
          <DialogHeader>
            <DialogTitle className="text-white">Create New Order</DialogTitle>
            <DialogDescription className="text-slate-400">
              Fill in the details to create a new order.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div>
              <Input
                placeholder="Product name"
                value={product}
                onChange={(e) => setProduct(e.target.value)}
                className="bg-white/5 border-white/10 text-white placeholder:text-slate-500"
              />
            </div>
            <div>
              <Input
                type="number"
                placeholder="Quantity"
                value={quantity}
                onChange={(e) => setQuantity(e.target.value)}
                className="bg-white/5 border-white/10 text-white placeholder:text-slate-500"
              />
            </div>

            {error && (
              <p className="text-sm text-red-400 bg-red-500/10 px-3 py-2 rounded-lg">{error}</p>
            )}

            <Button
              onClick={handleSubmit}
              disabled={loading}
              className="w-full"
            >
              {loading ? (
                <Icons.Loader className="size-4 animate-spin" />
              ) : (
                "Submit Order"
              )}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {result && (
        <div
          data-testid="order-success"
          className="p-4 rounded-xl bg-green-500/10 border border-green-500/20 space-y-3"
        >
          <div className="flex items-center gap-2">
            <div className="size-8 rounded-full bg-green-500/20 flex items-center justify-center">
              <Icons.Check className="size-4 text-green-400" />
            </div>
            <p className="font-medium text-green-400">Order Created Successfully!</p>
          </div>
          <div className="grid grid-cols-2 gap-3 text-sm">
            <div>
              <p className="text-slate-500 text-xs uppercase">Order ID</p>
              <p data-testid="order-id" className="font-mono text-white">
                {result.order_id.slice(0, 8)}...
              </p>
            </div>
            <div>
              <p className="text-slate-500 text-xs uppercase">Product</p>
              <p className="text-white">{result.product}</p>
            </div>
            <div>
              <p className="text-slate-500 text-xs uppercase">Status</p>
              <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs bg-green-500/20 text-green-400 capitalize">
                {result.status}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
