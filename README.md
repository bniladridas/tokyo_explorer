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
The temperature parameter <span style="font-family: 'Courier New', monospace;">T</span> controls the randomness of the AI's responses. A higher temperature results in more random outputs, while a lower temperature makes the output more deterministic.

The probability of selecting a token can be represented as:
<p style="font-family: 'Courier New', monospace;">
P(x) = \frac{e^{\frac{score(x)}{T}}}{\sum_{y \in V} e^{\frac{score(y)}{T}}}
</p>
where <span style="font-family: 'Courier New', monospace;">score(x)</span> is the score assigned to token <span style="font-family: 'Courier New', monospace;">x</span> and <span style="font-family: 'Courier New', monospace;">V</span> is the vocabulary.

### Statistical Sampling
**Top-k Sampling:**
In top-k sampling, the model selects from the top <span style="font-family: 'Courier New', monospace;">k</span> most probable tokens. The probability distribution is truncated to these tokens.

**Top-p Sampling (Nucleus Sampling):**
In top-p sampling, the model selects from the smallest set of tokens whose cumulative probability exceeds a threshold <span style="font-family: 'Courier New', monospace;">p</span>.

Mathematically, this can be represented as:
<p style="font-family: 'Courier New', monospace;">
Select S = \{x \in V \mid P(x) \geq \text{threshold} \}
</p>
where <span style="font-family: 'Courier New', monospace;">P(x)</span> is the probability of token <span style="font-family: 'Courier New', monospace;">x</span>.

### Bayesian Inference
Bayesian inference is a statistical framework for updating the probability of a hypothesis based on new evidence. It can be represented as:
<p style="font-family: 'Courier New', monospace;">
P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}
</p>
where <span style="font-family: 'Courier New', monospace;">P(H \mid E)</span> is the posterior probability of the hypothesis <span style="font-family: 'Courier New', monospace;">H</span> given the evidence <span style="font-family: 'Courier New', monospace;">E</span>, <span style="font-family: 'Courier New', monospace;">P(E \mid H)</span> is the likelihood of the evidence given the hypothesis, <span style="font-family: 'Courier New', monospace;">P(H)</span> is the prior probability of the hypothesis, and <span style="font-family: 'Courier New', monospace;">P(E)</span> is the marginal likelihood of the evidence.

### Markov Chain Monte Carlo (MCMC)
MCMC is a class of algorithms for sampling from a probability distribution. It can be represented as:
<p style="font-family: 'Courier New', monospace;">
\pi(x) = \lim_{t \to \infty} P(X_t = x)
</p>
where <span style="font-family: 'Courier New', monospace;">\pi(x)</span> is the stationary distribution of the Markov chain, <span style="font-family: 'Courier New', monospace;">X_t</span> is the state of the chain at time <span style="font-family: 'Courier New', monospace;">t</span>, and <span style="font-family: 'Courier New', monospace;">P(X_t = x)</span> is the probability of the chain being in state <span style="font-family: 'Courier New', monospace;">x</span> at time <span style="font-family: 'Courier New', monospace;">t</span>.
