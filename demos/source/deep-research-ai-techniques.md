# Deep Research with AI - Best Practices Report

**Date:** 2026-01-27
**Topic:** Deep research techniques with AI
**Source:** Community best practices synthesis

---

## Summary

Deep research with AI works best as an iterative process. The strongest workflows combine source checks, structured prompts, context building, confidence scoring, and review loops.

---

## Key Techniques

### 1. Iterative Research Loops

- Use AI to search, analyze, then search again with refined queries
- Chain multiple research steps instead of using one large prompt
- Let each iteration build on previous findings

### 2. Source Verification

- Cross-reference AI outputs with primary sources
- AI can hallucinate citations, so verify URLs and dates
- Prefer primary sources over summaries

### 3. Structured Research Frameworks

- Define research questions before starting
- Break broad topics into specific sub-queries
- Create a research plan with milestones

### 4. Context Building

- Feed useful background documents to AI before analysis
- Use retrieval-based workflows for domain-specific research
- Build a searchable knowledge base for complex topics

### 5. Prompt Engineering for Research

- Ask for sources with URLs in every research prompt
- Request confidence levels for claims and findings
- Ask for evidence and counterarguments

---

## Prompt Templates

### Basic Research Prompt

```text
Research [TOPIC]. For each claim, provide:
1. The finding
2. Source URL
3. Confidence level: high, medium, or low
4. Date of information
```

### Comparative Analysis Prompt

```text
Compare [TOPIC A] vs [TOPIC B]. Include:
- Key differences
- Use cases for each
- Trade-offs
- Recent developments
- Source URLs
```

---

## Workflow Example

```text
Step 1: Broad search
Step 2: Analyze and refine
Step 3: Deep dives
Step 4: Synthesis
Step 5: Verification
```

---

## Common Pitfalls

| Pitfall | Solution |
| --- | --- |
| Single-shot prompting | Use iterative loops |
| Accepting AI claims at face value | Verify sources |
| Too broad queries | Break into specific sub-queries |
| No confidence scoring | Ask for confidence levels |
| Missing dates | Request dates for all claims |

---

## Research Quality Checklist

- [ ] Research question clearly defined
- [ ] Multiple sources for each claim
- [ ] Primary sources preferred over summaries
- [ ] Dates included for all information
- [ ] Confidence levels assigned
- [ ] Contradictions noted and resolved
- [ ] Gaps identified for further research
- [ ] Sources verified

---

## References

- General deep research community practices
- Iterative research methodology from AI practitioners
- Chain-of-thought prompting research papers
- Retrieval-augmented generation best practices
