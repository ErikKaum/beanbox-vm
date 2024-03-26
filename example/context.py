from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains
import time


class Context:
    def __init__(self, addr: str):
        self.addr = addr

    def __enter__(self):
        self.driver = webdriver.Chrome()
        self.driver.get(self.addr)
        
        time.sleep(0.1) # to make sure the html has loaded
        
        self.terminal_element = self.driver.find_element(By.ID, 'terminal')
        self.driver.execute_script("arguments[0].focus();", self.terminal_element)
        self.actions = ActionChains(self.driver)

        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.driver.quit()

    def exec(self, cmd: str):
        for char in cmd:
            self.driver.execute_script(f"window.writeToTerminal('{char}')")
            time.sleep(0.05)

        self.actions.send_keys(Keys.RETURN).perform()

    def get_terminal_history(self) -> str:
        rows = self.terminal_element.find_elements(By.CSS_SELECTOR, '.xterm-rows > div')
        terminal_text = "\n".join([row.text for row in rows if row.text.strip() != ''])

        return terminal_text



