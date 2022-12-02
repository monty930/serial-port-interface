
// use std::time::Duration;

// fn main() {
//     let ports = serialport::available_ports().
//     expect("No ports found!");
//     for p in ports {
//         println!("{}", p.port_name);
//     }

//     let mut port =
//     serialport::new("/dev/ttyACM0", 115200)
//     .timeout(Duration::from_millis(10))
//     .open().expect("Failed to open port");

//     let output = "abc\n".as_bytes();
//     port.write(output).expect("Write failed!");

//     //let mut serial_buf: Vec<u8> = vec![0; 32];
//     //port.read(serial_buf.as_mut_slice()).expect("Found no data!");

//     //println!("{}", serial_buf[1]);

//     std::mem::drop(port);

//     println!("Hello, world!");
// }


// use std::io::{self, Write, Read};
// use std::panic::panic_any;
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
// // use std::io::{self, Read};

// fn main() {

//     let port = serialport::new("/dev/ttyACM0", 115200)
//         .timeout(Duration::from_millis(10))
//         .open();

//     match port {
//         Ok(mut port) => {
//             let mut serial_buf: Vec<u8> = vec![0; 1000];
//             println!("Receiving data on at baud:");
//             loop {
//                 match port.read(serial_buf.as_mut_slice()) {
//                     Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
//                     Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
//                     Err(e) => eprintln!("{:?}", e),
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("Failed to open Error: {}", e);
//             ::std::process::exit(1);
//         }
//     }
// }




// Provides a way to test clearing and querying the size of the serial output buffer.

// USAGE:

// 1. Connect a serial device to your host computer. E.g. an Arduino could be used. It will be able
//    to receive data without any specific sketch loaded.
// 2. Run this example
// 3. Observe the output - it reports how many bytes are waiting to be sent to the connected device
// 4. Press the Return key to make the example clear the output buffer. You should see the number
//    of bytes queued to send momentarily drop to 0
// 5. Try passing different values for the buffer-size argument to see how that affects the speed
//    and saturation point of the output buffer
// 6. Press Ctrl+D (Unix) or Ctrl+Z (Win) to quit


use std::error::Error;
use std::io::{self, Read, Write};
use std::panic::panic_any;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::time::Duration;

use serialport::ClearBuffer;

const DEFAULT_BLOCK_SIZE: usize = 128;

const HALF_SEC: Duration = time::Duration::from_millis(500);
const MINI_BREAK: Duration = time::Duration::from_millis(10);

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    };

    println!("Exit code: {}", exit_code);

    std::process::exit(exit_code);
}

fn run() -> Result<(), Box<dyn Error>> {

    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .map_err(|ref e| format!("Error. Failed to connect to the port /dev/ttyACM0: {}", e))?;

    //let chan_clear_buf = input_service();

    println!("Connected.");
    println!("Ctrl+D (Unix) or Ctrl+Z (Win) to stop. Press Return to clear the buffer.");

    //let block = vec![0; DEFAULT_BLOCK_SIZE];

    

    // This loop writes the block repeatedly, as fast as possible, to try to saturate the
    // output buffer. If you don't see much data queued to send, try changing the block size.
    loop {

        let mut serial_buf: Vec<u8> = vec![0; 1000];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        thread::sleep(HALF_SEC);

        let block2 = "a";
        match port.write(block2.as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error while writing data to the port: {}", e),
        };

        thread::sleep(MINI_BREAK);

        let block2 = "b";
        match port.write(block2.as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error while writing data to the port: {}", e),
        };

        thread::sleep(MINI_BREAK);

        let block2 = "c";
        match port.write(block2.as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error while writing data to the port: {}", e),
        };

        thread::sleep(MINI_BREAK);

        let block2 = "d";
        match port.write(block2.as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error while writing data to the port: {}", e),
        };

        println!("sent.");

        // match chan_clear_buf.try_recv() {
        //     Ok(_) => {
        //         println!("------------------------- Discarding buffer ------------------------- ");
        //         port.clear(ClearBuffer::Output)
        //             .expect("Failed to discard output buffer")
        //     }
        //     Err(mpsc::TryRecvError::Empty) => (),
        //     Err(mpsc::TryRecvError::Disconnected) => {
        //         println!("Stopping.");
        //         break;
        //     }
        // }

        // println!(
        //     "Bytes queued to send: {}",
        //     port.bytes_to_write().expect("Error calling bytes_to_write")
        // );
    }

    //Ok(())
}

// fn input_service() -> mpsc::Receiver<()> {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         println!("Thread created.");
//         let mut buffer = [0; 32];
//         loop {
//             // Block awaiting any user input
//             match io::stdin().read(&mut buffer) {
//                 Ok(0) => {
//                     println!("DEBUG1");
//                     drop(tx); // EOF, drop the channel and stop the thread
//                     break;
//                 }
//                 Ok(_) => {
//                     println!("DEBUG2");
//                     tx.send(()).unwrap() // Signal main to clear the buffer
//                 },
//                 Err(e) => panic_any(e),
//             }
//         }
//     });

//     rx
// }

// /* USER CODE BEGIN Header */
// /**
//   ******************************************************************************
//   * @file           : main.c
//   * @brief          : Main program body
//   ******************************************************************************
//   * @attention
//   *
//   * Copyright (c) 2022 STMicroelectronics.
//   * All rights reserved.
//   *
//   * This software is licensed under terms that can be found in the LICENSE file
//   * in the root directory of this software component.
//   * If no LICENSE file comes with this software, it is provided AS-IS.
//   *
//   ******************************************************************************
//   */
// /* USER CODE END Header */
// /* Includes ------------------------------------------------------------------*/
// #include "main.h"

// /* Private includes ----------------------------------------------------------*/
// /* USER CODE BEGIN Includes */

// #include <stdio.h>

// /* USER CODE END Includes */

// /* Private typedef -----------------------------------------------------------*/
// /* USER CODE BEGIN PTD */

// /* USER CODE END PTD */

// /* Private define ------------------------------------------------------------*/
// /* USER CODE BEGIN PD */
// /* USER CODE END PD */

// /* Private macro -------------------------------------------------------------*/
// /* USER CODE BEGIN PM */

// /* USER CODE END PM */

// /* Private variables ---------------------------------------------------------*/
// UART_HandleTypeDef huart2;
// DMA_HandleTypeDef hdma_usart2_rx;
// DMA_HandleTypeDef hdma_usart2_tx;

// /* USER CODE BEGIN PV */

// uint8_t Received[10];

// /* USER CODE END PV */

// /* Private function prototypes -----------------------------------------------*/
// void SystemClock_Config(void);
// static void MX_GPIO_Init(void);
// static void MX_DMA_Init(void);
// static void MX_USART2_UART_Init(void);
// /* USER CODE BEGIN PFP */

// void HAL_UART_RxCpltCallback(UART_HandleTypeDef *huart) {
 
// 	static uint8_t Data[40]; // Tablica przechowujaca wysylana wiadomosc.

//   if (Received[0] == 'a') {
//     HAL_GPIO_TogglePin(LD2_GPIO_Port, LD2_Pin);
//   }
 
// 	sprintf(Data, "Odebrana wiadomosc: %s\n\r", Received);
// 	HAL_UART_Transmit_DMA(&huart2, Data, 40); // Rozpoczecie nadawania danych z wykorzystaniem przerwan
// 	HAL_UART_Receive_DMA(&huart2, Received, 10); // Ponowne włączenie nasłuchiwania
	
// }

// /* USER CODE END PFP */

// /* Private user code ---------------------------------------------------------*/
// /* USER CODE BEGIN 0 */

// // #define LINE_MAX_LENGTH	80
 
// // static char line_buffer[LINE_MAX_LENGTH + 1];
// // static uint32_t line_length;
 
// // void line_append(uint8_t value)
// // {
// //   const char message[] = "Hello world!\r\n";
// //   HAL_UART_Transmit(&huart2, message, strlen(message), HAL_MAX_DELAY);  

// // 	if (value == '\r' || value == '\n') {
// // 		// odebraliśmy znak końca linii
// // 		if (line_length > 0) {
// // 			// jeśli bufor nie jest pusty to dodajemy 0 na końcu linii
// // 			line_buffer[line_length] = '\0';
// // 			// przetwarzamy dane
			
// //       if (line_buffer[0] == 'a' && line_buffer[1] == 'b' && line_buffer[2] == 'c' && line_buffer[3] == '\0') {
// //         HAL_GPIO_TogglePin(LD2_GPIO_Port, LD2_Pin);
// //       }

// // 			// zaczynamy zbieranie danych od nowa
// // 			line_length = 0;
// // 		}
// // 	}
// // 	else {
// // 		if (line_length >= LINE_MAX_LENGTH) {
// // 			// za dużo danych, usuwamy wszystko co odebraliśmy dotychczas
// // 			line_length = 0;
// // 		}
// // 		// dopisujemy wartość do bufora
// // 		line_buffer[line_length++] = value;
// // 	}
// // }

// /* USER CODE END 0 */

// /**
//   * @brief  The application entry point.
//   * @retval int
//   */
// int main(void)
// {
//   /* USER CODE BEGIN 1 */

//   /* USER CODE END 1 */

//   /* MCU Configuration--------------------------------------------------------*/

//   /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
//   HAL_Init();

//   /* USER CODE BEGIN Init */

//   /* USER CODE END Init */

//   /* Configure the system clock */
//   SystemClock_Config();

//   /* USER CODE BEGIN SysInit */

//   /* USER CODE END SysInit */

//   /* Initialize all configured peripherals */
//   MX_GPIO_Init();
//   MX_DMA_Init();
//   MX_USART2_UART_Init();
//   /* USER CODE BEGIN 2 */

//   HAL_UART_Receive_DMA(&huart2, Received, 10); // Rozpoczecie nasluchiwania na dane z wykorzystaniem DMA


//   // const char message[] = "START\r\n";
//   // HAL_UART_Transmit(&huart2, message, strlen(message), HAL_MAX_DELAY);  

//   /* USER CODE END 2 */

//   /* Infinite loop */
//   /* USER CODE BEGIN WHILE */
//   while (1)
//   {

//     // uint8_t value;
// 	  // if (HAL_UART_Receive(&huart2, &value, 1, 0) == HAL_OK) {
      
//     //   line_append(value);
//     // }

    
//     /* USER CODE END WHILE */

//     /* USER CODE BEGIN 3 */
//   }
//   /* USER CODE END 3 */
// }

// /**
//   * @brief System Clock Configuration
//   * @retval None
//   */
// void SystemClock_Config(void)
// {
//   RCC_OscInitTypeDef RCC_OscInitStruct = {0};
//   RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};

//   /** Configure the main internal regulator output voltage
//   */
//   __HAL_RCC_PWR_CLK_ENABLE();
//   __HAL_PWR_VOLTAGESCALING_CONFIG(PWR_REGULATOR_VOLTAGE_SCALE1);

//   /** Initializes the RCC Oscillators according to the specified parameters
//   * in the RCC_OscInitTypeDef structure.
//   */
//   RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI;
//   RCC_OscInitStruct.HSIState = RCC_HSI_ON;
//   RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
//   RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
//   RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSI;
//   RCC_OscInitStruct.PLL.PLLM = 16;
//   RCC_OscInitStruct.PLL.PLLN = 336;
//   RCC_OscInitStruct.PLL.PLLP = RCC_PLLP_DIV4;
//   RCC_OscInitStruct.PLL.PLLQ = 4;
//   if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
//   {
//     Error_Handler();
//   }

//   /** Initializes the CPU, AHB and APB buses clocks
//   */
//   RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK|RCC_CLOCKTYPE_SYSCLK
//                               |RCC_CLOCKTYPE_PCLK1|RCC_CLOCKTYPE_PCLK2;
//   RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
//   RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
//   RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV2;
//   RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;

//   if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_2) != HAL_OK)
//   {
//     Error_Handler();
//   }
// }

// /**
//   * @brief USART2 Initialization Function
//   * @param None
//   * @retval None
//   */
// static void MX_USART2_UART_Init(void)
// {

//   /* USER CODE BEGIN USART2_Init 0 */

//   /* USER CODE END USART2_Init 0 */

//   /* USER CODE BEGIN USART2_Init 1 */

//   /* USER CODE END USART2_Init 1 */
//   huart2.Instance = USART2;
//   huart2.Init.BaudRate = 115200;
//   huart2.Init.WordLength = UART_WORDLENGTH_8B;
//   huart2.Init.StopBits = UART_STOPBITS_1;
//   huart2.Init.Parity = UART_PARITY_NONE;
//   huart2.Init.Mode = UART_MODE_TX_RX;
//   huart2.Init.HwFlowCtl = UART_HWCONTROL_NONE;
//   huart2.Init.OverSampling = UART_OVERSAMPLING_16;
//   if (HAL_UART_Init(&huart2) != HAL_OK)
//   {
//     Error_Handler();
//   }
//   /* USER CODE BEGIN USART2_Init 2 */

//   /* USER CODE END USART2_Init 2 */

// }

// /**
//   * Enable DMA controller clock
//   */
// static void MX_DMA_Init(void)
// {

//   /* DMA controller clock enable */
//   __HAL_RCC_DMA1_CLK_ENABLE();

//   /* DMA interrupt init */
//   /* DMA1_Stream5_IRQn interrupt configuration */
//   HAL_NVIC_SetPriority(DMA1_Stream5_IRQn, 0, 0);
//   HAL_NVIC_EnableIRQ(DMA1_Stream5_IRQn);
//   /* DMA1_Stream6_IRQn interrupt configuration */
//   HAL_NVIC_SetPriority(DMA1_Stream6_IRQn, 0, 0);
//   HAL_NVIC_EnableIRQ(DMA1_Stream6_IRQn);

// }

// /**
//   * @brief GPIO Initialization Function
//   * @param None
//   * @retval None
//   */
// static void MX_GPIO_Init(void)
// {
//   GPIO_InitTypeDef GPIO_InitStruct = {0};

//   /* GPIO Ports Clock Enable */
//   __HAL_RCC_GPIOC_CLK_ENABLE();
//   __HAL_RCC_GPIOH_CLK_ENABLE();
//   __HAL_RCC_GPIOA_CLK_ENABLE();
//   __HAL_RCC_GPIOB_CLK_ENABLE();

//   /*Configure GPIO pin Output Level */
//   HAL_GPIO_WritePin(LD2_GPIO_Port, LD2_Pin, GPIO_PIN_RESET);

//   /*Configure GPIO pin : B1_Pin */
//   GPIO_InitStruct.Pin = B1_Pin;
//   GPIO_InitStruct.Mode = GPIO_MODE_IT_FALLING;
//   GPIO_InitStruct.Pull = GPIO_NOPULL;
//   HAL_GPIO_Init(B1_GPIO_Port, &GPIO_InitStruct);

//   /*Configure GPIO pin : LD2_Pin */
//   GPIO_InitStruct.Pin = LD2_Pin;
//   GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
//   GPIO_InitStruct.Pull = GPIO_NOPULL;
//   GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
//   HAL_GPIO_Init(LD2_GPIO_Port, &GPIO_InitStruct);

// }

// /* USER CODE BEGIN 4 */

// /* USER CODE END 4 */

// /**
//   * @brief  This function is executed in case of error occurrence.
//   * @retval None
//   */
// void Error_Handler(void)
// {
//   /* USER CODE BEGIN Error_Handler_Debug */
//   /* User can add his own implementation to report the HAL error return state */
//   __disable_irq();
//   while (1)
//   {
//   }
//   /* USER CODE END Error_Handler_Debug */
// }

// #ifdef  USE_FULL_ASSERT
// /**
//   * @brief  Reports the name of the source file and the source line number
//   *         where the assert_param error has occurred.
//   * @param  file: pointer to the source file name
//   * @param  line: assert_param error line source number
//   * @retval None
//   */
// void assert_failed(uint8_t *file, uint32_t line)
// {
//   /* USER CODE BEGIN 6 */
//   /* User can add his own implementation to report the file name and line number,
//      ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */
//   /* USER CODE END 6 */
// }
// #endif /* USE_FULL_ASSERT */
