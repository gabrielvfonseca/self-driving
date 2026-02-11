"""Main entry point for the AI planner service."""

import logging
import sys
import os

# Add the current directory to the path
sys.path.insert(0, os.path.dirname(__file__))

# Import modules directly from the current directory
from agents.planner_agent import PlannerAgent

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def main():
    """Main function to start the AI planner service."""
    logger.info("Starting AI Planner service")
    
    # Initialize the planner agent
    planner = PlannerAgent()
    
    logger.info("AI Planner service started successfully")
    return planner

if __name__ == "__main__":
    main()
