# Tokyo Explorer + AI

## Description
Tokyo Explorer + AI is a Rust application that provides an interactive experience for exploring Tokyo with the help of AI-generated content.

## Installation
To build the project, run the following command:

```bash
cargo build
```

## Usage
After building the project, you can run the application to explore Tokyo and ask questions about the city.

## Dependencies
This project uses the following dependencies:
- tokio
- colored
- rand
- dotenv
- reqwest
- serde
- serde_json

## Mathematical Concepts

### Probability Distributions
The temperature parameter \( T \) controls the randomness of the AI's responses. A higher temperature results in more random outputs, while a lower temperature makes the output more deterministic.

The probability of selecting a token can be represented as:
\[
P(x) = \frac{e^{\frac{score(x)}{T}}}{\sum_{y \in V} e^{\frac{score(y)}{T}}}
\]
where \( score(x) \) is the score assigned to token \( x \) and \( V \) is the vocabulary.

### Statistical Sampling
**Top-k Sampling:**
In top-k sampling, the model selects from the top \( k \) most probable tokens. The probability distribution is truncated to these tokens.

**Top-p Sampling (Nucleus Sampling):**
In top-p sampling, the model selects from the smallest set of tokens whose cumulative probability exceeds a threshold \( p \).

Mathematically, this can be represented as:
\[
\text{Select } S = \{x \in V \mid P(x) \geq \text{threshold} \}
\]
where \( P(x) \) is the probability of token \( x \).

### Bayesian Inference
Bayesian inference is a statistical framework for updating the probability of a hypothesis based on new evidence. It can be represented as:
\[
P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}
\]
where \( P(H \mid E) \) is the posterior probability of the hypothesis \( H \) given the evidence \( E \), \( P(E \mid H) \) is the likelihood of the evidence given the hypothesis, \( P(H) \) is the prior probability of the hypothesis, and \( P(E) \) is the marginal likelihood of the evidence.

### Markov Chain Monte Carlo (MCMC)
MCMC is a class of algorithms for sampling from a probability distribution. It can be represented as:
\[
\pi(x) = \lim_{t \to \infty} P(X_t = x)
\]
where \( \pi(x) \) is the stationary distribution of the Markov chain, \( X_t \) is the state of the chain at time \( t \), and \( P(X_t = x) \) is the probability of the chain being in state \( x \) at time \( t \).
