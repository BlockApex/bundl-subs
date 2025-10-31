# Bundl Subs
<p align="center">
  <a href="http://bundlsubs.com/" target="blank"><img src="assets/logo-with-text.png" width="240" alt="BundleSubs Logo" /></a>
</p>

**One payment. All your subscriptions. Powered by stablecoins.** No more random billing dates, surprise charges or failed payments.

<a href="https://arena.colosseum.org/projects/explore/bundl" target="blank"><img src="assets/colloseum.svg" style="width:22px; padding-right: 6px;" alt="Colloseum Logo"/></a>
<a href="https://www.legends.fun/products/f47715cc-cd61-414e-83b9-229786e48817" target="blank"><img src="assets/legends.png" style="width:22px;" alt="BundleSubs Logo"/></a>
[![X](assets/x.png)](https://x.com/bundlsubs)
[![Website](assets/globe.png)](https://bundlsubs.com)

## 🚀 Project Overview

**Bundl Subs** helps you fund and manage all your subscriptions in one place, unlocking exclusive discounts while offsetting costs with yield on idle assets.

Refer to the following submodules for their respective codebases: 
- **[Backend](backend/README.md)** (APIs and Cron Job)
- **[Frontend](frontend/README.md)** (PWA)
- **[Solana_Programs](solana-programs/README.md)** (Bundl program code and state)

> _Please see each submodule/directory for low-level implementation and API docs._


## 🧩 Product highlights

- **Bundle Creation/Discovery**: Users can create, discover, and subscribe to curated bundles
- **Secure Auto-Renewals**: Trust-minimized pull-based auto-renewing subscription payments on-chain
- **Exclusive deals and discounts**: Bundling creates stickiness, allowing merchants to offer exclusive discounts against reduces CAC.
- **Activity/Subscription Tracking**: Users track and manage one payment with an easy to use interface, can pause/resume or cancel anytime.


## ⚙️ How It Works

1. **User creates/discovers a bundle:** Server identifies the applicable offers from each choosen subscription and calculate their all-in monthly cost and bundle savings in real time.
1. **User deploys it's Subscription Controller:** A personalised controller will provide hassle-free pull-based auto-renewals while ensuring that the user only pays for bundles they have subscribed for.
1. **Backend pull payments every month:** Bundle server create invoices and pull payments for them in a trust-minimized fashion with split configuration.
1. **Merchants and users are communicated:** Every invoice creation and payment attempt is communicated to the merchants through webhook and to the users through email.
1. **User claim entitlements and enjoy their subscriptions:** Users can claim entitlement for a service included in their bundle in variety of ways, configurable on merchants side. An activation link on email, a form submission with dynamic fields or a merchant code

![User Flow](assets/high-level-diagram.png)

> _Sounds interesting? wanna dig deep? navigate to the relevant submodule to find lower level details and specifications._

## 🔗 Data Flow Diagram

![Data Flow](assets/data-flow.png)

## Subscription Activation Sequence Diagram

![Subscription Sequence Diagram](assets/payment-sequence-diagram.png)

## 🛠️ Tech Stack

- **Frontend:** Next.js (React), TypeScript
- **Backend:** NestJS, TypeScript, MongoDB
- **Smart Contracts:** Rust, Anchor, Solana

## Product Roadmap
- Build Privacy Stack to limit both user-merchant & merchant-merchant visibility on-chain.
- Build Merchant Portal for merchants to on-board and track subscriptions on user
- Smart Yield to offset subscriptions with yield
- Allow importing existing subscriptions for more inclusivity
- Traditional card to enable bundling web2 subscriptions that don't accept crypto payments