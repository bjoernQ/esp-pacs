#[doc = "Register `RTC_GPIO_PIN%s` reader"]
pub struct R(crate::R<RTC_GPIO_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_PIN%s` writer"]
pub struct W(crate::W<RTC_GPIO_PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_GPIO_PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PIN0_PAD_DRIVER` reader - Pad driver selection. 0: normal output. 1: open drain."]
pub type GPIO_PIN0_PAD_DRIVER_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN0_PAD_DRIVER` writer - Pad driver selection. 0: normal output. 1: open drain."]
pub type GPIO_PIN0_PAD_DRIVER_W<'a> = crate::BitWriter<'a, u32, RTC_GPIO_PIN_SPEC, bool, 2>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN0_INT_TYPE_W<'a> = crate::FieldWriter<'a, u32, RTC_GPIO_PIN_SPEC, u8, u8, 3, 7>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a> = crate::BitWriter<'a, u32, RTC_GPIO_PIN_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn gpio_pin0_pad_driver(&self) -> GPIO_PIN0_PAD_DRIVER_R {
        GPIO_PIN0_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn gpio_pin0_pad_driver(&mut self) -> GPIO_PIN0_PAD_DRIVER_W {
        GPIO_PIN0_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W {
        GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W {
        GPIO_PIN0_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC configuration for pin %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_pin](index.html) module"]
pub struct RTC_GPIO_PIN_SPEC;
impl crate::RegisterSpec for RTC_GPIO_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_pin::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_pin::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_PIN%s to value 0"]
impl crate::Resettable for RTC_GPIO_PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
