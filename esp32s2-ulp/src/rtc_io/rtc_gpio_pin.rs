#[doc = "Register `RTC_GPIO_PIN%s` reader"]
pub type R = crate::R<RTC_GPIO_PIN_SPEC>;
#[doc = "Register `RTC_GPIO_PIN%s` writer"]
pub type W = crate::W<RTC_GPIO_PIN_SPEC>;
#[doc = "Field `GPIO_PIN_PAD_DRIVER` reader - Pad driver selection. 0: normal output. 1: open drain."]
pub type GPIO_PIN_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `GPIO_PIN_PAD_DRIVER` writer - Pad driver selection. 0: normal output. 1: open drain."]
pub type GPIO_PIN_PAD_DRIVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIO_PIN_INT_TYPE` reader - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN_INT_TYPE` writer - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GPIO_PIN_WAKEUP_ENABLE` reader - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN_WAKEUP_ENABLE` writer - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN_WAKEUP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn gpio_pin_pad_driver(&self) -> GPIO_PIN_PAD_DRIVER_R {
        GPIO_PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin_int_type(&self) -> GPIO_PIN_INT_TYPE_R {
        GPIO_PIN_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin_wakeup_enable(&self) -> GPIO_PIN_WAKEUP_ENABLE_R {
        GPIO_PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_PIN")
            .field(
                "gpio_pin_pad_driver",
                &format_args!("{}", self.gpio_pin_pad_driver().bit()),
            )
            .field(
                "gpio_pin_int_type",
                &format_args!("{}", self.gpio_pin_int_type().bits()),
            )
            .field(
                "gpio_pin_wakeup_enable",
                &format_args!("{}", self.gpio_pin_wakeup_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_pad_driver(&mut self) -> GPIO_PIN_PAD_DRIVER_W<RTC_GPIO_PIN_SPEC, 2> {
        GPIO_PIN_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_int_type(&mut self) -> GPIO_PIN_INT_TYPE_W<RTC_GPIO_PIN_SPEC, 7> {
        GPIO_PIN_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_wakeup_enable(&mut self) -> GPIO_PIN_WAKEUP_ENABLE_W<RTC_GPIO_PIN_SPEC, 10> {
        GPIO_PIN_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC configuration for pin %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_pin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_pin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_PIN_SPEC;
impl crate::RegisterSpec for RTC_GPIO_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_pin::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_pin::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_PIN%s to value 0"]
impl crate::Resettable for RTC_GPIO_PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
