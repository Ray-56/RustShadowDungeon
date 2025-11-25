//! # Rust Shadow Dungeon (锈影地下城)
//!
//! A DNF-inspired 2D action game built with Rust and Bevy.
//!
//! ## Architecture
//!
//! This codebase follows Domain-Driven Design (DDD) principles:
//!
//! - **Domain Layer** (`domain/`): Pure Rust business logic, zero Bevy dependencies
//! - **Infrastructure Layer** (`infrastructure/`): Bevy ECS bridge (components, systems, plugins)
//!
//! ## Constitution Compliance
//!
//! This project adheres to Constitution v1.0.1:
//! - Principle I: Rust Memory Safety (zero unsafe code)
//! - Principle II: Bevy ECS Architecture
//! - Principle III: 60 FPS Performance Target
//! - Principle IV: Pixel Art Consistency (16×16 grid)
//! - Principle V: Combat Mechanics Testing (≥85% coverage)
//! - Principle VI: Open Source MIT License
//! - Principle VII: Modular Design
//! - Principle VIII: Language Separation (code in English, docs/comments in Chinese when needed)

#![warn(missing_docs, clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(clippy::needless_pass_by_value)] // Bevy systems use Res<T> by value
#![allow(clippy::module_name_repetitions)] // Common in DDD (e.g. VelocityComponent)
#![allow(clippy::multiple_crate_versions)] // Dependency management is hard
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate, clippy::missing_errors_doc)]

// Domain layer - pure Rust, zero Bevy dependencies
pub mod domain;

// Infrastructure layer - Bevy ECS bridge
pub mod infrastructure;
