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
The temperature parameter T controls the randomness of the AI's responses. A higher temperature results in more random outputs, while a lower temperature makes the output more deterministic.

```mermaid
graph TD;
    A[Probability of selecting a token] --> B[Score of token x];
    B --> C[Temperature T];
    C --> D[Probability distribution];
```

### Statistical Sampling
**Top-k Sampling:**
In top-k sampling, the model selects from the top k most probable tokens. The probability distribution is truncated to these tokens.

**Top-p Sampling (Nucleus Sampling):**
In top-p sampling, the model selects from the smallest set of tokens whose cumulative probability exceeds a threshold p.

```mermaid
graph TD;
    A[Top-k Sampling] --> B[Top k most probable tokens];
    A --> C[Truncated probability distribution];
    D[Top-p Sampling] --> E[Smallest set of tokens];
    E --> F[Cumulative probability exceeds threshold p];
```

### Bayesian Inference
Bayesian inference is a statistical framework for updating the probability of a hypothesis based on new evidence.

```mermaid
graph TD;
    A[Bayesian Inference] --> B[Posterior probability];
    B --> C[Likelihood];
    C --> D[Prior probability];
    D --> E[Marginal likelihood];
```

### Markov Chain Monte Carlo (MCMC)
MCMC is a class of algorithms for sampling from a probability distribution.

```mermaid
graph TD;
    A[MCMC] --> B[Stationary distribution];
    B --> C[State of the chain at time t];
    C --> D[Probability of the chain being in state x at time t];
```

<script type="text/javascript" async
  src="https://cdnjs.cloudflare.com/ajax/libs/mermaid/8.13.10/mermaid.min.js">
</script>
