#!/usr/bin/env python3
"""Main entry point for the AI Planner service."""

import logging
from ai_planner.agents.planner_agent import PlannerAgent

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

def main():
    """Main entry point."""
    logger.info("Starting AI Planner service")
    
    # Initialize the planner agent
    agent = PlannerAgent()
    
    # Start the agent
    agent.start()
    
    logger.info("AI Planner service started successfully")

if __name__ == "__main__":
    main()
