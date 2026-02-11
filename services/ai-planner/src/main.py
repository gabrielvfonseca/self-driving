"""
Main entry point for the AI Planner service
"""
import logging
from ai_planner.agents.planner_agent import PlannerAgent

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def main():
    """Main function to start the AI Planner service"""
    logger.info("Starting AI Planner service")
    
    # Initialize the planner agent
    planner = PlannerAgent()
    
    logger.info("AI Planner service started successfully")
    return planner

if __name__ == "__main__":
    main()
